use crate::base::*;
use super::row::*;
use super::cons::*;
use async_std::sync::{ Sender };

pub struct ExternalChoice < Row >
where
  Row : RowCon,
{ pub sender :
    Sender <
      ( AppliedSum < Row, () >,
        Sender <
          AppliedSum < Row, ReceiverApp >
        >
      )
    >
}

impl < Row >
  Protocol for
  ExternalChoice < Row >
where
  Row : Send + 'static,
  Row : RowCon,
{ }

impl < Row, A >
  RecApp < A > for
  ExternalChoice < Row >
where
  Row : RecApp < A >,
  Row : RowCon,
  Row::Applied : RowCon,
{
  type Applied =
    ExternalChoice <
      Row::Applied
    >;
}
