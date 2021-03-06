use async_macros::join;

use async_std::task;
use async_std::sync::{ channel };

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
    async move | ctx, sender1 | {
      let (sender2, receiver) = channel(1);

      let child1 = task::spawn(async move {
        let val = receiver.recv().await.unwrap();
        sender1.send (
          Wrap { unwrap : Box::new ( val ) }
        ).await;
      });

      let child2 = task::spawn(
        unsafe_run_session
          ( cont, ctx, sender2
          ) );

      join!(child1, child2).await;
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
    async move | ctx1, sender1 | {
      let (receiver1, ctx2) = N :: extract_source ( ctx1 );

      let (sender2, receiver2) = channel(1);

      let ctx3 =
        N :: insert_target ( receiver2, ctx2 );

      let child1 = task::spawn ( async move {
        let wrapped = receiver1.recv().await.unwrap();
        sender2.send( *wrapped.unwrap ).await;
      });

      let child2 = task::spawn(
        unsafe_run_session
          ( cont, ctx3, sender1
          ));

      join!(child1, child2).await;
    })
}
