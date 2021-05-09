use std::marker::PhantomData;

use super::{
  super::cloak_session::*,
  traits::*,
};
use crate::internal::{
  base::*,
  functional::*,
};

pub struct InjectSessionF<Row, C>(PhantomData<(Row, C)>);

pub struct InjectSession<Row, C, A>
{
  injector : Box<dyn SessionInjector<Row, C, A>>,
}

pub fn create_inject_session<Row, C, A, I>(
  injector : I
) -> InjectSession<Row, C, A>
where
  I : SessionInjector<Row, C, A> + 'static,
{
  InjectSession {
    injector : Box::new(injector),
  }
}

pub fn run_inject_session<Row, C, A>(
  inject : InjectSession<Row, C, A>,
  session : PartialSession<C, A>,
) -> AppSum<Row, SessionF<C>>
where
  C : Context,
  A : Protocol,
{
  inject.injector.inject_session(session)
}