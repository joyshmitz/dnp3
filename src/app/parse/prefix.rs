use crate::app::parse::traits::FixedSize;
use crate::util::cursor::{ReadCursor, ReadError};

#[derive(Debug, PartialEq)]
pub struct Prefix<I, V>
where
    I: FixedSize,
    V: FixedSize,
{
    pub index: I,
    pub value: V,
}

impl<I, V> FixedSize for Prefix<I, V>
where
    I: FixedSize,
    V: FixedSize,
{
    const SIZE: u8 = I::SIZE + V::SIZE;

    fn read(cursor: &mut ReadCursor) -> Result<Self, ReadError> {
        Ok(Prefix {
            index: I::read(cursor)?,
            value: V::read(cursor)?,
        })
    }
}
