use crate::{Error, Result};
use serde::ser::{self, Serialize};

pub fn to_binary<S: Serialize>(data: S) -> Result<Vec<u8>> {
    let mut serializer = Serializer::new();

    data.serialize(&mut serializer)?;
    Ok(serializer.container)
}

pub struct Serializer {
    container: Vec<u8>,
}

impl Serializer {
    pub fn new() -> Self {
        Self {
            container: Vec::new(),
        }
    }

    fn push_bytes(&mut self, bytes: &[u8]) -> () {
        self.container.extend_from_slice(bytes);
    }

    fn push_str(&mut self, string: String) -> () {
        self.push_bytes(string.as_bytes())
    }

    pub(crate) fn ser_number(&mut self, number: impl Into<i64>) -> () {
        self.container.push(b'i');
        self.push_str(number.into().to_string());
        self.container.push(b'e');
    }

    pub(crate) fn ser_bytes(&mut self, bytes: &[u8]) -> () {
        self.push_str(bytes.len().to_string());
        self.container.push(b':');
        self.push_bytes(bytes);
    }
    pub(crate) fn ser_string(&mut self, string: &str) -> () {
        self.ser_bytes(string.as_bytes())
    }
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    type SerializeSeq = Self;

    type SerializeTuple = Self;

    type SerializeTupleStruct = Self;

    type SerializeTupleVariant = Self;

    type SerializeMap = Self;

    type SerializeStruct = Self;

    type SerializeStructVariant = Self;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok> {
        Err(Error::TypeNotSupported)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        Ok(self.ser_number(v))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        Ok(self.ser_number(v))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        Ok(self.ser_number(v))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        Ok(self.ser_number(v))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        Ok(self.ser_number(v))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        Ok(self.ser_number(v))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        Ok(self.ser_number(v))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        // FIXME: Error::Syntax????
        let v: i64 = v.try_into().map_err(|_| Error::Syntax)?;

        Ok(self.ser_number(v))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        Err(Error::TypeNotSupported)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        Err(Error::TypeNotSupported)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        // TODO: maybe allow one-symbol string
        Err(Error::TypeNotSupported)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        Ok(self.ser_string(v))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        Ok(self.ser_bytes(v))
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        // TODO: think about it
        Ok(())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_newtype_struct<T: ?Sized>(self, name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.container.push(b'l');
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        todo!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        todo!()
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        todo!()
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> std::result::Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        self.container.push(b'e');
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> std::result::Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> std::result::Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> std::result::Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> std::result::Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> std::result::Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> std::result::Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> std::result::Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> std::result::Result<Self::Ok, Self::Error> {
        todo!()
    }
}
