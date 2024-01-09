use std::collections::BTreeMap;

use crate::types;
use serde::de;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Value {
    Number(types::Number),
    String(types::String),
    List(types::List<Value>),
    Dictionary(types::Dictionary<Value>),
    Bytes(Vec<u8>),
}

impl<'de> de::Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ValueVisitor;
        impl<'de> de::Visitor<'de> for ValueVisitor {
            type Value = Value;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "valid barcode")
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> {
                Ok(Value::Number(v))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E> {
                Ok(Value::String(v))
            }

            fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E> {
                Ok(Value::Bytes(v))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let mut vec = Vec::new();

                while let Some(v) = seq.next_element()? {
                    vec.push(v);
                }

                Ok(Value::List(vec))
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                let mut bmap = BTreeMap::new();

                while let Some((key, value)) = map.next_entry()? {
                    bmap.insert(key, value);
                }

                Ok(Value::Dictionary(bmap))
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::Value::*;
    use crate::from_binary;

    crate::macros::parse_test! {
        test_number: _ => (b"i4e" == Number(4));
        test_string: _ => (b"4:test" == String("test".to_string()));
        test_list: _ => (b"l3:foo3:bare" == List(vec![String("foo".to_string()), String("bar".to_string())]));
        test_dictionary: _ => (b"d4:spaml1:a1:bee" == {
            let mut map = BTreeMap::new();
            map.insert(
                "spam".to_string(),
                List(vec![String("a".to_string()), String("b".to_string())]),
            );
            Dictionary(map)
        })
    }
}
