#[derive(Debug, Default)]
pub struct RawRow {
    count: usize,
    pub value_list: Vec<crate::Value>,
}

impl RawRow {
    pub fn new(count: usize) -> Self {
        RawRow {
            count,
            value_list: vec![crate::Value::NULL; count],
        }
    }
    pub fn update_value(&mut self, index: usize, value: crate::Value) -> &mut Self {
        if index < self.count {
            self.value_list[index] = value;
        }
        self
    }
}

pub trait Row: std::fmt::Debug + std::default::Default {
    fn update_field(&mut self, column_name: &str, value: crate::Value) -> &mut Self;
}
