#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub enum ObjectCategory{
    Invalid,
    Piece,
    Collection,
}

impl ObjectCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            ObjectCategory::Piece => "piece",
            ObjectCategory::Collection => "collection",
            ObjectCategory::Invalid => "invalid",
        }
    }
    pub fn from_str(s: &str) -> ObjectCategory {
        match s {
            "piece" => ObjectCategory::Piece,
            "collection" => ObjectCategory::Collection,
            _ => ObjectCategory::Invalid,
        }
    }
}

pub struct ObjectType {
    name: String,
    category: ObjectCategory,
}


mod object_type;
pub mod db;
mod code;

pub fn get_object_type_by_id() {
    object_type::get_by_id();
}