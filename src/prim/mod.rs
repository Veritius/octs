mod bool;
mod nonzero;
mod num;

use core::{convert::Infallible, fmt::Display};

use crate::BufTooShortOr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InvalidValue;

impl Display for InvalidValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "invalid value")
    }
}

impl From<InvalidValue> for BufTooShortOr<InvalidValue> {
    fn from(value: InvalidValue) -> Self {
        Self::Or(value)
    }
}

impl From<BufTooShortOr<Infallible>> for BufTooShortOr<InvalidValue> {
    fn from(value: BufTooShortOr<Infallible>) -> Self {
        match value {
            BufTooShortOr::TooShort => Self::TooShort,
            BufTooShortOr::Or(_) => unreachable!(),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for InvalidValue {}
