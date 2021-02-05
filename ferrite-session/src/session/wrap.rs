use async_macros::join;

use crate::base::*;
use crate::protocol::*;

pub fn wrap_session
  < C, T >
  ( cont :
      PartialSession <
        C,
        T :: Unwrap
      >
  ) ->
    PartialSession <
      C,
      Wrap < T >
    >
where
  C : Context,
  T : Wrapper,
  T : Send + 'static,
  T :: Unwrap : Protocol,
{
  unsafe_create_session (
    move | ctx, sender1 | async move {
      let (sender2, receiver) = once_channel();

      let child1 = spawn(async move {
        let val = receiver.recv().await.unwrap();
        sender1.send (
          Wrap { unwrap : Box::new ( val ) }
        ).await.unwrap();
      });

      let child2 = spawn(
        unsafe_run_session
          ( cont, ctx, sender2
          ) );

      let _ = join!(child1, child2).await;
    })
}

pub fn unwrap_session
  < N, C, T, A >
  ( _ : N,
    cont :
      PartialSession <
        N :: Target,
        A
      >
  ) ->
    PartialSession < C, A >
where
  C : Context,
  A : Protocol,
  T : Wrapper + Send + 'static,
  N :
    ContextLens <
      C,
      Wrap < T >,
      T :: Unwrap
    >
{
  unsafe_create_session(
    move | ctx1, sender1 | async move {
      let (receiver1, ctx2) = N :: extract_source ( ctx1 );

      let (sender2, receiver2) = once_channel();

      let ctx3 =
        N :: insert_target ( receiver2, ctx2 );

      let child1 = spawn ( async move {
        let wrapped = receiver1.recv().await.unwrap();
        sender2.send( *wrapped.unwrap ).await.unwrap();
      });

      let child2 = spawn(
        unsafe_run_session
          ( cont, ctx3, sender1
          ));

      let _ = join!(child1, child2).await;
    })
}
