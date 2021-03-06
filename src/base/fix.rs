use std::mem::transmute;
use std::marker::PhantomData;

use crate::base::nat::*;

pub trait TypeApp < A > {
  type Applied;
}

pub struct Unfix < A > ( PhantomData<A> );

pub struct Fix < F >
{
  unfix : Box < F >
}

pub fn fix < F >
  (x : F :: Applied)
  -> Fix < F >
where
  F :
    TypeApp <
      Unfix <
        Fix < F >
      >
    >
{
  unsafe {
    let wrapped : Box < F > =
      transmute ( Box::new ( x ) );

    Fix {
      unfix : wrapped
    }
  }
}

pub fn unfix < F >
  (x : Fix < F >)
  -> F :: Applied
where
  F :
    TypeApp <
      Unfix <
        Fix < F >
      >
    >
{
  unsafe {
    let wrapped : Box < F::Applied > =
      transmute ( x.unfix );

    *wrapped
  }
}

impl < A, F >
  TypeApp < A >
  for Fix < F >
where
  F :
    TypeApp <
      S < A >
    >,
  F :
    TypeApp < Unfix <
      Fix < F >
    > >,
  < F as
    TypeApp <
      S < A >
    >
  > :: Applied :
    TypeApp < Unfix <
      Fix <
        < F as
          TypeApp <
            S < A >
          >
        > :: Applied
      >
    > >,
{
  type Applied =
    Fix <
      < F as
        TypeApp <
          S < A >
        >
      > :: Applied
    >;
}

impl < A >
  TypeApp < Unfix < A > > for
  Z
{
  type Applied = A;
}

impl
  TypeApp < Z > for
  Z
{
  type Applied = Z;
}

impl < A >
  TypeApp < S < A > > for
  Z
{
  type Applied = Z;
}

impl < A, N >
  TypeApp < S < A > > for
  S < N >
where
  N : TypeApp < A >
{
  type Applied = S < N::Applied >;
}

impl < A, N >
  TypeApp < Unfix < A > > for
  S < N >
{
  type Applied = N;
}

impl < N >
  TypeApp < Z > for
  S < N >
{
  type Applied = S < N >;
}

impl < A >
  TypeApp < A > for
  ()
{
  type Applied = ();
}

impl < A, X, Y >
  TypeApp < A > for
  ( X, Y )
where
  X : TypeApp < A >,
  Y : TypeApp < A >,
{
  type Applied =
    ( X :: Applied,
      Y :: Applied
    );
}
