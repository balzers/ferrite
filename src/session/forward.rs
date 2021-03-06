
use crate::base::{
  Protocol,
  Empty,
  Context,
  EmptyContext,
  ContextLens,
  PartialSession,
  unsafe_create_session,
};

pub fn forward
  < N, C, A >
  (_ : N)
  ->
    PartialSession <
      C,
      A
    >
where
  A : Protocol,
  C : Context,
  N :: Target : EmptyContext,
  N :
    ContextLens <
      C,
      A,
      Empty
    >
{
  unsafe_create_session (
    async move | ctx, sender | {
      let (receiver, _) = N :: extract_source ( ctx );

      let val = receiver.recv().await.unwrap();
      sender.send( val ).await;
    })
}
