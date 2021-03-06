use async_macros::join;

use crate::base::*;
use async_std::task;
use async_std::sync::{ Sender, channel };

pub fn fix_session
  < F, A, C >
  ( cont: PartialSession < C, A > )
  ->
    PartialSession <
      C,
      Fix < F >
    >
where
  C : Context,
  F : Protocol,
  A : Protocol,
  F :
    TypeApp <
      Unfix <
        Fix < F >
      >,
      Applied = A
    >,
{
  unsafe_create_session (
    async move | ctx, sender1 | {
      let (sender2, receiver)
        : ( Sender < A >, _ )
        = channel(1);

      let child1 = task::spawn(async move {
        let val = receiver.recv().await.unwrap();
        sender1.send ( fix ( val ) ).await;
      });

      let child2 = task::spawn(
        unsafe_run_session
          ( cont, ctx, sender2 ) );

      join!(child1, child2).await;
    })
}

pub fn unfix_session
  < C, F, A >
  ( cont:
      PartialSession <
        C,
        Fix < F >
      >
  ) ->
    PartialSession < C, A >
where
  C : Context,
  F : Protocol,
  A : Protocol,
  F :
    TypeApp <
      Unfix <
        Fix < F >
      >,
      Applied = A
    >,
{
  unsafe_create_session (
    async move | ctx, sender1 | {
      let (sender2, receiver) = channel(1);

      let child1 = task::spawn(async move {
        let val = receiver.recv().await.unwrap();
        sender1.send ( unfix ( val ) ).await;
      });

      let child2 = task::spawn(
        unsafe_run_session
          ( cont, ctx, sender2
          ) );

      join!(child1, child2).await;
    })
}

pub fn succ_session
  < I, P >
  ( cont : PartialSession < I, P > )
  -> PartialSession < I, S < P > >
where
  P : Protocol,
  I : Context,
{
  unsafe_create_session (
    async move | ctx, sender | {
      let (sender2, receiver) = channel(1);

      let child1 = task::spawn(async move {
        let val = receiver.recv().await.unwrap();
        sender.send ( succ ( val ) ).await;
      });

      let child2 = task::spawn(
        unsafe_run_session
          ( cont, ctx, sender2
          ) );

      join!(child1, child2).await;
    })
}

pub fn unfix_session_for
  < N, C, A, B, F >
  ( _ : N,
    cont :
      PartialSession <
        N :: Target,
        B
      >
  ) ->
    PartialSession < C, B >
where
  B : Protocol,
  C : Context,
  F : Protocol,
  F :
    TypeApp <
      Unfix <
        Fix < F >
      >,
      Applied = A
    >,
  A : Protocol,
  N :
    ContextLens <
      C,
      Fix < F >,
      A,
    >,
{
  unsafe_create_session(
    async move | ctx1, sender1 | {
      let (receiver1, ctx2) =
        N :: extract_source ( ctx1 );

        let (sender2, receiver2) = channel(1);

      let ctx3 =
        N :: insert_target ( receiver2, ctx2 );

      let child1 = task::spawn ( async move {
        let val = receiver1.recv().await.unwrap();
        sender2.send( unfix ( val ) ).await;
      });

      let child2 = task::spawn(
        unsafe_run_session
          ( cont, ctx3, sender1
          ));

      join!(child1, child2).await;
    })
}
