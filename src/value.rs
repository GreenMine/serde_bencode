use std::collections::BTreeMap;

use crate::types;
use serde::{de, ser};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    Number(types::Number),
    String(types::String),
    Bytes(Vec<u8>),
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

impl ser::Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Value::Number(n) => serializer.serialize_i64(*n),
            Value::String(str) => serializer.serialize_str(str),
            Value::Bytes(bytes) => serializer.serialize_bytes(bytes),
            Value::List(list) => list.serialize(serializer),
            Value::Dictionary(dict) => dict.serialize(serializer),
        }
    }
}
