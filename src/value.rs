use std::collections::BTreeMap;

use crate::types;
use serde::de;

// TODO: thing about adding T generic, cuz bencode can contains only single type of data in
// collections(maybe can collect different, idk)
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Value {
    Number(types::Number),
    String(types::String),
    List(types::List<Value>),
    Dictionary(types::Dictionary<Value>),
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

    #[test]
    pub fn test_number() {
        let j = b"i4e";
        let expected = Number(4);

        assert_eq!(expected, from_binary(j).unwrap())
    }

    #[test]
    pub fn test_string() {
        let j = b"4:test";
        let expected = String("test".to_string());

        assert_eq!(expected, dbg!(from_binary(j).unwrap()))
    }

    #[test]
    pub fn test_list() {
        let j = b"l3:foo3:bare";
        let expected = List(vec![String("foo".to_string()), String("bar".to_string())]);

        assert_eq!(expected, from_binary(j).unwrap());
    }

    #[test]
    pub fn test_dictionary() {
        let j = b"d4:spaml1:a1:bee";

        let mut map = BTreeMap::new();
        map.insert(
            "spam".to_string(),
            List(vec![String("a".to_string()), String("b".to_string())]),
        );
        let expected = Dictionary(map);

        assert_eq!(expected, from_binary(j).unwrap());
    }
}
