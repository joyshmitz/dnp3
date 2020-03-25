use crate::app::parse::traits::FixedSize;
use crate::util::cursor::{ReadCursor, ReadError};

#[derive(Debug, PartialEq)]
pub struct InvalidRange;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Range {
    start: u16,
    count: usize,
}

impl Range {
    /// construct a range in a way that ensures only a valid range can be obtained
    pub fn from(start: u16, stop: u16) -> Result<Self, InvalidRange> {
        if stop < start {
            return Err(InvalidRange);
        }

        Ok(Self {
            start,
            count: stop as usize - start as usize + 1,
        })
    }

    pub fn empty() -> Self {
        Self { start: 0, count: 0 }
    }

    pub fn get_start(&self) -> u16 {
        self.start
    }

    pub fn get_count(&self) -> usize {
        self.count
    }
}

#[derive(Debug, PartialEq)]
pub struct RangedSequence<'a, T>
where
    T: FixedSize,
{
    start: u16,
    data: &'a [u8],
    phantom: std::marker::PhantomData<T>,
}

impl<'a, T> RangedSequence<'a, T>
where
    T: FixedSize,
{
    pub fn parse(
        range: Range,
        cursor: &mut ReadCursor<'a>,
    ) -> Result<RangedSequence<'a, T>, ReadError> {
        // this cannot overflow b/c SIZE is [0, 255] and count is [0, 65536]
        let num_bytes = T::SIZE as usize * range.count;
        Ok(Self::new(range.start, cursor.read_bytes(num_bytes)?))
    }

    pub fn empty() -> Self {
        Self::new(0, &[])
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn new(start: u16, data: &'a [u8]) -> Self {
        Self {
            start,
            data,
            phantom: std::marker::PhantomData {},
        }
    }

    pub fn iter(&self) -> RangeIterator<'a, T> {
        RangeIterator {
            index: self.start,
            cursor: ReadCursor::new(self.data),
            phantom: std::marker::PhantomData {},
        }
    }
}

pub struct RangeIterator<'a, T> {
    index: u16,
    cursor: ReadCursor<'a>,
    phantom: std::marker::PhantomData<T>,
}

impl<'a, T> Iterator for RangeIterator<'a, T>
where
    T: FixedSize,
{
    type Item = (T, u16);

    fn next(&mut self) -> Option<Self::Item> {
        match T::read(&mut self.cursor) {
            Ok(x) => {
                let idx = self.index;
                self.index += 1;
                Some((x, idx))
            }
            Err(_) => None,
        }
    }
}
