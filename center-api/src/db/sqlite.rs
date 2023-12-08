use sqlite;
use serde::{Serialize, Deserialize};

use crate::code::Code;


#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    code: Code,
    msg: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Field {
    Integer(i64),
    Real(f64),
    Text(String),
}

pub struct RawRows {
    pub head_list: Vec<String>,
    pub rows: Vec<Vec<Field>>,
}

impl RawRows {
    fn check(&self) -> bool {
        if self.head_list.len() == 0 {
            return false;
        }
        if self.rows.len() == 0 {
            return false;
        }
        for row in self.rows.iter() {
            if row.len() != self.head_list.len() {
                return false;
            }
        }
        true
    }
    pub fn new(head_list: Vec<String>) -> RawRows {
        RawRows {
            head_list,
            rows: Vec::new(),
        }
    }
    pub fn add_row(&mut self, row: Vec<Field>) {
        self.rows.push(row);
    }

    pub fn insert(&self, conn: sqlite::Connection, table_name: &str) -> Result<(), Error> {
        let header_str = self.head_list.join(",");
        let mut value_str = String::new();
        for row in self.rows.iter() {
            let mut row_str = String::new();
            for field in row.iter() {
                match field {
                    Field::Integer(i) => {
                        row_str.push_str(&i.to_string());
                    }
                    Field::Real(f) => {
                        row_str.push_str(&f.to_string());
                    }
                    Field::Text(s) => {
                        row_str.push_str(&s);
                    }
                }
                row_str.push_str(",");
            }
            row_str.pop();
            value_str.push_str(&format!("({}),", row_str));
            value_str.pop();
        }
        let sql = format!("INSERT INTO {} ({}) VALUES {};",
                          table_name, header_str, value_str);
        let result = conn.execute(sql);
        if let Err(e) = result {
            return Err(Error {
                code: Code::DBError,
                msg: format!("Unable to insert data to table: {}, error: {}",
                             table_name, e.to_string()),
            });
        }
        Ok(())
    }
}

