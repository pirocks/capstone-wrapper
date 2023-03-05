use std::num::{NonZeroU8, ParseIntError};
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(PartialEq, Eq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
#[serde(transparent)]
#[repr(transparent)]
pub struct OperandIndex(pub NonZeroU8);


#[derive(Error, Debug)]
pub enum OperandIndexError {
    #[error(transparent)]
    Parse(#[from] ParseIntError),
    #[error("Operand index was zero")]
    IndexIsZero,
}

impl FromStr for OperandIndex {
    type Err = OperandIndexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(OperandIndex(NonZeroU8::new(u8::from_str(s)?).ok_or(OperandIndexError::IndexIsZero)?))
    }
}


