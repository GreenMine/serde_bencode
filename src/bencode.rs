pub mod types {
    pub type Number = i64;
    pub type String = std::string::String;
    pub type List<T> = Vec<T>;
    pub type Dictionary<T> = std::collections::HashMap<String, T>;
}

pub enum Value<T> {
    Number(types::Number),
    String(types::String),
    // TODO: types::List<Value>
    List(types::List<T>),
    Dictionary(types::Dictionary<T>),
}
