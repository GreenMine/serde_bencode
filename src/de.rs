use crate::bencode::{self, types};
use crate::stream::BinaryStream;
use crate::{Error, Result};

use serde::{de, Deserialize};

pub struct Deserializer<'de> {
    input: BinaryStream<'de>,
}

pub fn from_binary<'a, T: Deserialize<'a>>(data: &'a [u8]) -> Result<T> {
    let mut deserializer = Deserializer::new(data);

    T::deserialize(&mut deserializer)
}

impl<'de> Deserializer<'de> {
    pub fn new(data: &'de [u8]) -> Self {
        Self {
            input: BinaryStream::new(data),
        }
    }

    fn parse_seq_number(&mut self, terminator: u8) -> Result<types::Number> {
        fn to_num(n: u8) -> Option<i64> {
            Some((n as char).to_digit(10)? as i64)
        }

        let mut init = 0i64;
        let next = self.input.try_next()?;
        if next != b'-' {
            init = to_num(next).ok_or(Error::ExpectedNumber)?;
        }

        let mut result = self
            .input
            .take_while(|&v| v != terminator)
            .try_fold(init, |acc, v| {
                let v = to_num(v).ok_or(Error::ExpectedNumber)?;

                Ok(acc * 10 + v)
            })?;

        if init == 0 {
            result *= -1;
        }

        Ok(result)
    }

    pub(crate) fn parse_numeric<T: TryFrom<i64>>(&mut self) -> Result<T> {
        if self.input.try_next()? != b'i' {
            return Err(Error::ExpectedNumber);
        }

        return self
            .parse_seq_number(b'e')?
            .try_into()
            .map_err(|_| Error::ExpectedNumber);
    }

    pub(crate) fn parse_string(&mut self) -> Result<types::String> {
        let len = self.parse_seq_number(b':')?;

        // TODO: validate size before take
        Ok(self
            .input
            .take(len as usize)
            .map(|v| v as char)
            .collect::<String>())
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i8<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i8(self.parse_numeric()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i16(self.parse_numeric()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i32(self.parse_numeric()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i64(self.parse_numeric()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u8(self.parse_numeric()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u16(self.parse_numeric()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u32(self.parse_numeric()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u64(self.parse_numeric()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_f64<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_char<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_string<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_string(self.parse_string()?)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if self.input.try_next()? == b'l' {
            let value = visitor.visit_seq(BencodeList { de: self })?;

            if self.input.try_next()? == b'e' {
                return Ok(value);
            } else {
                Err(Error::ExpectedEnd)
            }
        } else {
            Err(Error::ExpectedList)
        }
    }

    fn deserialize_tuple<V>(
        self,
        len: usize,
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }
}

struct BencodeList<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> de::SeqAccess<'de> for BencodeList<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(
        &mut self,
        seed: T,
    ) -> std::result::Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        if self.de.input.try_peek()? == b'e' {
            return Ok(None);
        }

        seed.deserialize(&mut *self.de).map(Some)
    }
}

#[cfg(test)]
mod tests {
    use super::from_binary;
    use serde_derive::Deserialize;

    #[test]
    pub fn test_num() {
        #[derive(Deserialize, Debug, PartialEq)]
        struct Baz {
            foo: u64,
            bar: u64,
        }

        let j = b"i-1e";
        let expected = -1;

        assert_eq!(expected, from_binary::<i16>(j).unwrap());
    }

    #[test]
    pub fn test_string() {
        let j = b"3:foo";
        let expected = "foo";

        assert_eq!(expected, from_binary::<String>(j).unwrap());
    }

    #[test]
    pub fn test_empty_list() {
        let j = b"le";
        let expected: Vec<String> = Vec::new();

        assert_eq!(expected, from_binary::<Vec<String>>(j).unwrap())
    }

    #[test]
    pub fn test_list() {
        let j = b"l4:spam4:eggse";
        let expected: Vec<String> = vec!["spam".to_string(), "eggs".to_string()];

        assert_eq!(expected, from_binary::<Vec<String>>(j).unwrap())
    }

    #[test]
    pub fn test_tuple() {
        let j = b"l4:spam4:eggs4:lulwe";
        let expected: (String, String) = ("spam".to_string(), "eggs".to_string());

        assert_eq!(expected, from_binary(j).unwrap());
    }
}
