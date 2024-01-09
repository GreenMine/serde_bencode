use crate::stream::BinaryStream;
use crate::types;
use crate::{Error, Result};

use serde::{de, Deserialize};

pub struct Deserializer<'de> {
    input: BinaryStream<'de>,
}

pub fn from_binary<'a, T: Deserialize<'a>>(data: &'a [u8]) -> Result<T> {
    let mut deserializer = Deserializer::new(data);

    // TODO: check trailing characters
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
            init = to_num(next).ok_or(Error::Syntax)?;
        }

        // TODO: if sequence just end(eof), error not be,
        // which may cause logical bug
        let mut result = self
            .input
            .take_while(|&v| v != terminator)
            .try_fold(init, |acc, v| {
                let v = to_num(v).ok_or(Error::Syntax)?;

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

    pub(crate) fn parse_str(&mut self) -> Result<&'de str> {
        let bytes = self.parse_bytes()?;
        Self::from_utf8(bytes)
    }

    fn from_utf8(bytes: &[u8]) -> Result<&str> {
        std::str::from_utf8(bytes).map_err(|_| Error::InvalidString)
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.input.try_peek()? {
            b'0'..=b'9' => {
                let bytes = self.parse_bytes()?;

                match Deserializer::from_utf8(bytes) {
                    Ok(str) => visitor.visit_string(str.to_owned()),
                    Err(Error::InvalidString) => visitor.visit_byte_buf(bytes.to_owned()),
                    Err(e) => Err(e),
                }
            }
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
        Err(Error::TypeNotSupported)
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
        Err(Error::TypeNotSupported)
    }

    fn deserialize_f64<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::TypeNotSupported)
    }

    fn deserialize_char<V>(self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::TypeNotSupported)
    }

    fn deserialize_str<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_borrowed_str(self.parse_str()?)
    }

    fn deserialize_string<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_borrowed_bytes(self.parse_bytes()?)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
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
        self.deserialize_str(visitor)
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

    crate::macros::parse_test! {
        test_string: String => (b"3:foo" == "foo".to_string());
        test_empty_list: Vec<String> => (b"le" == Vec::new());
        test_list: Vec<String> => (b"l4:spam4:eggse" == vec!["spam".to_string(), "eggs".to_string()]);
        test_tuple: (String, String) => (b"l4:spam4:eggse" == ("spam".to_string(), "eggs".to_string()));
        test_empty_dictionary: HashMap<String, i32> => (b"de" == HashMap::new());
        test_dictionary: HashMap<String, u8> => (b"d3:onei1e3:twoi2e5:threei3e4:fouri4ee" == {
            let mut map = HashMap::new();
            map.insert("one".to_string(), 1);
            map.insert("two".to_string(), 2);
            map.insert("three".to_string(), 3);
            map.insert("four".to_string(), 4);
            map
        });
        test_list_in_dictionary: _ => (b"d4:spaml1:a1:bee" == {
            let mut map = HashMap::new();
            map.insert("spam".to_string(), vec!["a".to_string(), "b".to_string()]);
            map
        });
        test_bytes: &[u8] => (b"4:asdf" == b"asdf");
        test_bytes_list: Vec<&[u8]> => (b"l4:teste" == vec![&b"test"[..]]);
        test_borrow_str: &str => (b"4:meta" == "meta")
    }

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
}
