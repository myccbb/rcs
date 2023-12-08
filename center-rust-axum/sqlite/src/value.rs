#[derive(Clone, Debug)]
pub enum Value {
    NULL,
    Integer(i64),
    Real(f64),
    Text(String),
    Blob(Vec<u8>),
}

impl ValueType for Value {
    fn value(&self) -> &Value {
        self
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Value::Integer(v as i64)
    }
}

impl From<Value> for bool {
    fn from(v: Value) -> Self {
        match v {
            Value::NULL => false,
            Value::Integer(i) => i != 0,
            Value::Real(r) => r != 0.0,
            Value::Text(t) => t.parse::<i64>().unwrap() != 0,
            Value::Blob(b) => !b.is_empty(),
        }
    }
}

impl From<u32> for Value {
    fn from(v: u32) -> Self {
        Value::Integer(v.into())
    }
}

impl From<i8> for Value {
    fn from(v: i8) -> Self {
        Value::Integer(v.into())
    }
}

impl From<i16> for Value {
    fn from(v: i16) -> Self {
        Value::Integer(v.into())
    }
}

impl From<i32> for Value {
    fn from(v: i32) -> Self {
        Value::Integer(v.into())
    }
}

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Value::Integer(v)
    }
}

impl From<Value> for i64 {
    fn from(v: Value) -> i64 {
        match v {
            Value::NULL => 0,
            Value::Integer(i) => i,
            Value::Real(r) => r as i64,
            Value::Text(t) => t.parse::<i64>().unwrap(),
            Value::Blob(b) => b.len() as i64,
        }
    }
}

impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Value::Real(v)
    }
}

impl From<Value> for f64 {
    fn from(v: Value) -> f64 {
        match v {
            Value::NULL => 0.0,
            Value::Integer(i) => i as f64,
            Value::Real(r) => r,
            Value::Text(t) => t.parse::<f64>().unwrap(),
            Value::Blob(b) => b.len() as f64,
        }
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Value::Text(v)
    }
}

impl From<Value> for String {
    fn from(v: Value) -> Self {
        match v {
            Value::NULL => String::from(""),
            Value::Integer(i) => i.to_string(),
            Value::Real(r) => r.to_string(),
            Value::Text(t) => t,
            Value::Blob(b) => String::from_utf8(b).unwrap(),
        }
    }
}

impl From<&String> for Value {
    fn from(v: &String) -> Self {
        Value::Text(v.clone())
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Value::Text(v.to_string())
    }
}

impl From<Vec<u8>> for Value {
    fn from(v: Vec<u8>) -> Self {
        Value::Blob(v)
    }
}

impl From<chrono::DateTime<chrono::FixedOffset>> for Value {
    fn from(v: chrono::DateTime<chrono::FixedOffset>) -> Self {
        v.to_rfc3339().into()
    }
}

impl From<chrono::DateTime<chrono::Utc>> for Value {
    fn from(v: chrono::DateTime<chrono::Utc>) -> Self {
        v.to_rfc3339().into()
    }
}

impl From<Value> for chrono::DateTime<chrono::FixedOffset> {
    fn from(value: Value) -> Self {
        chrono::DateTime::parse_from_rfc3339(&String::from(value)).unwrap_or_default()
    }
}

impl From<Value> for chrono::DateTime<chrono::Utc> {
    fn from(value: Value) -> Self {
        chrono::DateTime::parse_from_rfc3339(&String::from(value))
            .unwrap_or_default()
            .into()
    }
}

pub trait ValueType {
    fn value(&self) -> &Value;
}
