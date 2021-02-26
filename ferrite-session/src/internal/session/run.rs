use async_macros::join;
use tokio::task;

use crate::internal::{
  base::{
    once_channel,
    unbounded,
    unsafe_create_shared_channel,
    unsafe_run_session,
    unsafe_run_shared_session,
    Session,
    SharedChannel,
    SharedProtocol,
    SharedSession,
    Value,
  },
  protocol::{
    End,
    SendValue,
  },
};

pub async fn run_session(session : Session<End>)
{
  let (sender, receiver) = once_channel();

  let child1 = task::spawn(async move {
    unsafe_run_session(session, (), sender).await;
  });

  let child2 = task::spawn(async move {
    receiver.recv().await.unwrap();
  });

  let _ = join!(child1, child2).await;
}

pub async fn run_session_with_result<T>(
  session : Session<SendValue<T, End>>
) -> T
where
  T : Send + 'static,
{
  let (sender, receiver1) = once_channel();

  let child1 = task::spawn(async move {
    unsafe_run_session(session, (), sender).await;
  });

  let SendValue((Value(val), receiver2)) = receiver1.recv().await.unwrap();

  receiver2.recv().await.unwrap();

  let _ = child1.await;

  val
}

pub fn run_shared_session<A>(session : SharedSession<A>) -> SharedChannel<A>
where
  A : SharedProtocol,
{
  let (chan, _) = run_shared_session_with_join_handle(session);

  chan
}

pub fn run_shared_session_with_join_handle<A>(
  session : SharedSession<A>
) -> (SharedChannel<A>, task::JoinHandle<()>)
where
  A : SharedProtocol,
{
  let (sender1, receiver1) = unbounded();

  let (session2, receiver2) = unsafe_create_shared_channel();

  task::spawn(async move {
    info!("[run_shared_session] exec_shared_session");

    unsafe_run_shared_session(session, receiver1).await;

    info!("[run_shared_session] exec_shared_session returned");
  });

  let handle = task::spawn(async move {
    loop {
      let m_senders = receiver2.recv().await;

      debug!("[run_shared_session] received sender3");

      match m_senders {
        Some(senders) => {
          sender1.send(senders).unwrap();
        }
        None => {
          info!("[run_shared_session] terminating shared session");

          return;
        }
      }
    }
  });

  (session2, handle)
}
