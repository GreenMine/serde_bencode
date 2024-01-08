use crate::stream::BinaryStream;
use crate::types;
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

        self.parse_seq_number(b'e')?
            .try_into()
            .map_err(|_| Error::ExpectedNumber)
    }

    pub(crate) fn parse_bytes(&mut self) -> Result<&'de [u8]> {
        let len = self.parse_seq_number(b':')? as usize;

        self.input.try_take(len)
    }

    pub(crate) fn parse_string(&mut self) -> Result<types::String> {
        Ok(self
            .parse_bytes()?
            .into_iter()
            .map(|&v| v as char)
            .collect::<String>())
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.input.try_peek()? {
            b'0'..=b'9' => self.deserialize_string(visitor),
            b'i' => self.deserialize_i64(visitor),
            b'l' => self.deserialize_seq(visitor),
            b'd' => self.deserialize_map(visitor),
            _ => Err(Error::Syntax),
        }
    }

    fn deserialize_bool<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
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

    fn deserialize_f32<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_f64<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_char<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::BorrowStr)
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
        visitor.visit_borrowed_bytes(self.parse_bytes()?)
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
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
            let value = visitor.visit_seq(BencodeCollection { de: self })?;

            if self.input.try_next()? == b'e' {
                Ok(value)
            } else {
                Err(Error::ExpectedEnd)
            }
        } else {
            Err(Error::ExpectedList)
        }
    }

    fn deserialize_tuple<V>(
        self,
        _len: usize,
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
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
        if self.input.try_next()? == b'd' {
            let value = visitor.visit_map(BencodeCollection { de: self })?;

            if self.input.try_next()? == b'e' {
                Ok(value)
            } else {
                Err(Error::ExpectedEnd)
            }
        } else {
            Err(Error::ExpectedDictionary)
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
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
        self.deserialize_string(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

struct BencodeCollection<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> de::SeqAccess<'de> for BencodeCollection<'a, 'de> {
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

impl<'a, 'de> de::MapAccess<'de> for BencodeCollection<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> std::result::Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        if self.de.input.try_peek()? == b'e' {
            return Ok(None);
        }

        seed.deserialize(&mut *self.de).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }
}

#[cfg(test)]
mod tests {
    use super::from_binary;
    use serde_derive::Deserialize;
    use std::collections::HashMap;

    #[test]
    pub fn test_struct() {
        #[derive(Deserialize, Debug, PartialEq)]
        struct Baz {
            foo: u64,
            bar: u64,
        }

        let j = b"d3:fooi255e3:bari1023e1:zi1ee";
        let expected = Baz {
            foo: 255,
            bar: 1023,
        };

        assert_eq!(expected, from_binary::<Baz>(j).unwrap());
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
        let j = b"l4:spam4:eggse";
        let expected: (String, String) = ("spam".to_string(), "eggs".to_string());

        assert_eq!(expected, from_binary(j).unwrap());
    }

    #[test]
    pub fn test_empty_dictionary() {
        let j = b"de";
        let expected: HashMap<String, i32> = HashMap::new();

        assert_eq!(expected, from_binary(j).unwrap());
    }

    #[test]
    pub fn test_dictionary() {
        let j = b"d3:onei1e3:twoi2e5:threei3e4:fouri4ee";
        let mut expected: HashMap<String, u8> = HashMap::new();
        expected.insert("one".to_string(), 1);
        expected.insert("two".to_string(), 2);
        expected.insert("three".to_string(), 3);
        expected.insert("four".to_string(), 4);

        assert_eq!(expected, from_binary(j).unwrap());
    }

    #[test]
    pub fn test_list_in_dictionary() {
        let j = b"d4:spaml1:a1:bee";

        let mut expected = HashMap::new();
        expected.insert("spam".to_string(), vec!["a".to_string(), "b".to_string()]);

        assert_eq!(expected, from_binary(j).unwrap());
    }

    #[test]
    pub fn test_borrow_str() {
        let j = b"4:meta";

        assert!(matches!(
            from_binary::<&str>(j),
            Err(crate::Error::BorrowStr)
        ));
    }

    #[test]
    pub fn test_bytes() {
        let j = b"4:asdf";
        let expected = b"asdf";

        assert_eq!(expected, from_binary::<&[u8]>(j).unwrap());
    }

    #[test]
    pub fn test_bytes_list() {
        let j = b"l4:teste";
        let expected = vec![&b"test"[..]];

        assert_eq!(expected, from_binary::<Vec<&[u8]>>(j).unwrap());
    }
}
