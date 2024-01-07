type Unit = u8;
type UnitContainer<'a> = &'a [Unit];

use crate::{Error, Result};

pub struct BinaryStream<'s> {
    data: UnitContainer<'s>,
    p: usize,
}

impl<'s> BinaryStream<'s> {
    pub fn new(data: UnitContainer<'s>) -> Self {
        Self { data, p: 0 }
    }

    pub fn try_peek(&self) -> Result<Unit> {
        if self.p >= self.data.len() {
            return Err(Error::Eof);
        }

        Ok(self.data[self.p])
    }

    pub fn try_next(&mut self) -> Result<Unit> {
        let v = self.try_peek()?;
        self.p += 1;

        Ok(v)
    }
}

impl<'s> Iterator for &mut BinaryStream<'s> {
    type Item = Unit;

    fn next(&mut self) -> Option<Self::Item> {
        self.try_next().ok()
    }
}
