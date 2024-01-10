type Unit = u8;
type UnitContainer<'a> = &'a [Unit];

use crate::{Error, Result};

pub struct ReadBinaryStream<'s> {
    data: UnitContainer<'s>,
    p: usize,
}

impl<'s> ReadBinaryStream<'s> {
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

    pub fn try_take(&mut self, n: usize) -> Result<&'s [Unit]> {
        let new_p = self.p + n;
        if new_p > self.data.len() {
            return Err(Error::Eof);
        }

        let slice = &self.data[self.p..new_p];
        self.p = new_p;

        Ok(slice)
    }
}

impl<'s> Iterator for &mut ReadBinaryStream<'s> {
    type Item = Unit;

    fn next(&mut self) -> Option<Self::Item> {
        self.try_next().ok()
    }
}
