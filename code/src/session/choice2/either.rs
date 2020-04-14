pub use crate::base::{
  TyApp
};

pub use crate::processes::*;
pub use crate::process::choice2::*;

pub type EitherField < A, B, T > =
  Either <
    < T as TyApp<A> > :: Applied,
    < T as TyApp<B> > :: Applied
  >;

pub enum Either < A, B > {
  Left ( A ),
  Right ( B ),
}

impl < T, A, B >
  SumRow < T > for
  Either < A, B >
where
  T : TyApp < A >,
  T : TyApp < B >,
  < T as TyApp<A> > :: Applied : Send,
  < T as TyApp<B> > :: Applied : Send,
{
  type Field = Either <
    < T as TyApp<A> > :: Applied,
    < T as TyApp<B> > :: Applied
  >;
}

impl < A, B >
  Iso
  for Either < A, B >
{
  type Canon = ( A, ( B, () ) );
}

impl < A, B, T >
  IsoRow < T >
  for Either < A, B >
where
  T : TyApp < A >,
  T : TyApp < B >,
  < T as TyApp<A> > :: Applied : Send,
  < T as TyApp<B> > :: Applied : Send,
{
  fn to_canon (
    row : EitherField < A, B, T >
  ) ->
    < Self :: Canon
      as SumRow < T >
    > :: Field
  {
    match row {
      Either::Left ( a ) => {
        Sum::Inl ( a )
      },
      Either::Right ( a ) => {
        Sum::Inr (
          Sum::Inl ( a ) )
      }
    }
  }

  fn from_canon (
    row :
      < Self :: Canon
        as SumRow < T >
      > :: Field
  ) ->
    EitherField < A, B, T >
  {
    match row {
      Sum::Inl ( a ) => {
        Either::Left( a )
      },
      Sum::Inr ( row2 ) => {
        match row2 {
          Sum::Inl ( a ) => {
            Either::Right( a )
          },
          Sum::Inr ( bot ) => {
            match bot {}
          }
        }
      }
    }
  }
}