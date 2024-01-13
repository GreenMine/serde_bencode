use std::collections::BTreeMap;

use crate::types;
use serde::{de, ser};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ValueRef<'a> {
    Number(types::Number),
    String(&'a str),
    Bytes(&'a [u8]),
    List(types::List<ValueRef<'a>>),
    Dictionary(types::Dictionary<ValueRef<'a>>),
}

impl<'de: 'a, 'a> de::Deserialize<'de> for ValueRef<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ValueVisitor;
        impl<'de> de::Visitor<'de> for ValueVisitor {
            type Value = ValueRef<'de>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "valid barcode")
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> {
                Ok(ValueRef::Number(v))
            }

            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ValueRef::String(v))
            }

            fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(ValueRef::Bytes(v))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let mut vec = Vec::new();

                while let Some(v) = seq.next_element()? {
                    vec.push(v);
                }

                Ok(ValueRef::List(vec))
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                let mut bmap = BTreeMap::new();

                while let Some((key, value)) = map.next_entry()? {
                    bmap.insert(key, value);
                }

                Ok(ValueRef::Dictionary(bmap))
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}

impl<'a> ser::Serialize for ValueRef<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ValueRef::Number(n) => serializer.serialize_i64(*n),
            ValueRef::String(str) => serializer.serialize_str(str),
            ValueRef::Bytes(bytes) => serializer.serialize_bytes(bytes),
            ValueRef::List(list) => list.serialize(serializer),
            ValueRef::Dictionary(dict) => dict.serialize(serializer),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ValueRef;
    use crate::{from_binary, to_binary};
    use serde_derive::Deserialize;

    #[test]
    fn test_value_ref() {
        let content = b"d4:info4:lulwe";
        #[derive(Deserialize, Debug)]
        struct InfoOnlyTorrent<'a> {
            #[serde(borrow)]
            info: ValueRef<'a>,
        }
        let info: InfoOnlyTorrent = from_binary(content).unwrap();
        println!("Info: {info:#?}");

        let info_raw = to_binary(&info.info).unwrap();

        println!("Raw: {:?}", std::str::from_utf8(&info_raw).unwrap());
    }
}
