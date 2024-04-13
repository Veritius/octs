use core::convert::Infallible;

use crate::{BufTooShortOr, Decode, Encode, FixedEncodeLen, Read, Write};

use super::InvalidValue;

impl FixedEncodeLen for bool {
    const ENCODE_LEN: usize = 1;
}

impl Decode for bool {
    type Error = InvalidValue;

    #[inline]
    fn decode(src: &mut impl Read) -> Result<Self, BufTooShortOr<Self::Error>> {
        match src.read_exact::<1>()? {
            [0] => Ok(false),
            [1] => Ok(true),
            [_] => Err(InvalidValue.into()),
        }
    }
}

impl Encode for bool {
    type Error = Infallible;

    #[inline]
    fn encode(&self, dst: &mut impl Write) -> Result<(), BufTooShortOr<Self::Error>> {
        dst.write(&u8::from(*self))
    }
}
