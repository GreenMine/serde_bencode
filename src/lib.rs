mod de;
mod err;
mod ser;
mod value;
mod value_ref;

pub(crate) mod stream;

pub use de::from_binary;
pub use err::{Error, Result};
pub use ser::to_binary;
pub use value::Value;
pub use value_ref::ValueRef;

pub mod types {
    pub type Number = i64;
    pub type String = std::string::String;
    pub type List<T> = Vec<T>;
    pub type Dictionary<T> = std::collections::BTreeMap<String, T>;
}
