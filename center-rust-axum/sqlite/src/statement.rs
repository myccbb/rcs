use libsqlite3_sys as sqlite_lib;

pub struct Statement {
    stmt: *mut sqlite_lib::sqlite3_stmt,
}

impl Statement {
    pub fn new(stmt: *mut sqlite_lib::sqlite3_stmt) -> Self {
        Self { stmt }
    }

    pub fn bind(&self, v: Vec<crate::Value>) -> Result<(), crate::Error> {
        for (i, v) in v.into_iter().enumerate() {
            self.bind_one(v, (i + 1) as i32)?;
        }
        Ok(())
    }

    fn bind_one(&self, v: crate::Value, nth: i32) -> Result<(), crate::Error> {
        let res_code = match v {
            crate::Value::NULL => unsafe { sqlite_lib::sqlite3_bind_null(self.stmt, nth) },
            crate::Value::Integer(i) => unsafe {
                sqlite_lib::sqlite3_bind_int64(self.stmt, nth, i)
            },
            crate::Value::Real(f) => unsafe { sqlite_lib::sqlite3_bind_double(self.stmt, nth, f) },
            crate::Value::Text(s) => unsafe {
                let c_str = std::ffi::CString::new(s)?;
                sqlite_lib::sqlite3_bind_text(
                    self.stmt,
                    nth,
                    c_str.as_ptr(),
                    c_str.as_bytes().len() as i32,
                    sqlite_lib::SQLITE_TRANSIENT(),
                )
            },
            crate::Value::Blob(b) => unsafe {
                sqlite_lib::sqlite3_bind_blob(
                    self.stmt,
                    nth,
                    b.as_ptr() as *const std::ffi::c_void,
                    b.len() as i32,
                    None,
                )
            },
        };
        if res_code != sqlite_lib::SQLITE_OK {
            return Err(crate::Error::BindFail(
                res_code,
                sqlite_lib::code_to_str(res_code).into(),
            ));
        }
        Ok(())
    }
    pub fn step(&self) -> Result<Option<()>, crate::Error> {
        let res_code;
        unsafe {
            res_code = sqlite_lib::sqlite3_step(self.stmt);
        }
        match res_code {
            sqlite_lib::SQLITE_ROW => Ok(Some(())),
            sqlite_lib::SQLITE_DONE => Ok(None),
            sqlite_lib::SQLITE_CONSTRAINT_PRIMARYKEY => Err(crate::Error::ConstraintPrimaryKey),
            sqlite_lib::SQLITE_CONSTRAINT_UNIQUE => Err(crate::Error::ConstraintUnique),
            _ => Err(crate::Error::StepFail(
                res_code,
                sqlite_lib::code_to_str(res_code).into(),
            )),
        }
    }
    pub fn finalize(&mut self) {
        unsafe {
            sqlite_lib::sqlite3_finalize(self.stmt);
        }
        self.stmt = std::ptr::null_mut();
    }

    pub fn get_one_raw_row(&self) -> Result<Option<crate::RawRow>, crate::Error> {
        let column_count: i32;
        let mut row;
        unsafe {
            column_count = sqlite_lib::sqlite3_column_count(self.stmt);
            if column_count <= 0 {
                return Ok(None);
            }
            row = crate::RawRow::new(column_count as usize);
            for i in 0..column_count {
                match sqlite_lib::sqlite3_column_type(self.stmt, i) {
                    sqlite_lib::SQLITE_NULL => {
                        row.update_value(i as usize, crate::Value::NULL);
                    }
                    sqlite_lib::SQLITE_INTEGER => {
                        let value = sqlite_lib::sqlite3_column_int64(self.stmt, i);
                        row.update_value(i as usize, crate::Value::Integer(value));
                    }
                    sqlite_lib::SQLITE_FLOAT => {
                        let value = sqlite_lib::sqlite3_column_double(self.stmt, i);
                        row.update_value(i as usize, crate::Value::Real(value));
                    }
                    sqlite_lib::SQLITE_TEXT => {
                        let text_ptr = sqlite_lib::sqlite3_column_text(self.stmt, i);
                        let text_len = sqlite_lib::sqlite3_column_bytes(self.stmt, i);
                        let u8_ptr = std::slice::from_raw_parts(text_ptr, (text_len) as usize);
                        let str = String::from_utf8(u8_ptr.into());
                        if let Err(e) = str {
                            let ptr = sqlite_lib::sqlite3_column_name(self.stmt, i);
                            let c_slice = std::ffi::CStr::from_ptr(ptr).to_bytes();
                            let column_name = String::from_utf8_lossy(c_slice).to_string();
                            return Err(crate::Error::TextIsNotUtf8(column_name, e.to_string()));
                        }
                        let str = str.unwrap().to_string();
                        row.update_value(i as usize, crate::Value::Text(str));
                    }
                    _ => {
                        return Err(crate::Error::InvalidColumnType(i));
                    }
                }
            }
        }
        Ok(Some(row))
    }
    pub fn get_all_raw_rows(&self) -> Result<Vec<crate::RawRow>, crate::Error> {
        let mut results = Vec::new();
        loop {
            let result = self.step()?;
            if result.is_none() {
                break;
            }
            let result = self.get_one_raw_row()?;
            if result.is_none() {
                break;
            }
            results.push(result.unwrap());
        }
        Ok(results)
    }

    pub fn get_column_name_list(&self, mut column_count: Option<i32>) -> Vec<String> {
        if column_count.is_none() {
            unsafe {
                column_count = Some(sqlite_lib::sqlite3_column_count(self.stmt));
            }
        }
        let column_count = column_count.unwrap();
        if column_count <= 0 {
            return vec![];
        }
        let mut column_name_list: Vec<String> = vec![];
        for i in 0..column_count {
            let ptr;
            unsafe {
                ptr = sqlite_lib::sqlite3_column_name(self.stmt, i);
            }
            let c_slice = unsafe { std::ffi::CStr::from_ptr(ptr).to_bytes() };
            let str = String::from_utf8_lossy(c_slice).to_string();
            column_name_list.push(str);
        }
        column_name_list
    }
    pub fn get_one_row<T>(&self) -> Result<Option<T>, crate::Error>
    where
        T: crate::Row,
    {
        let mut vt;
        unsafe {
            let column_count = sqlite_lib::sqlite3_column_count(self.stmt);
            if column_count <= 0 {
                return Ok(None);
            }
            let column_name_list = self.get_column_name_list(Some(column_count));
            vt = T::default();
            for i in 0..column_count {
                match sqlite_lib::sqlite3_column_type(self.stmt, i) {
                    sqlite_lib::SQLITE_NULL => {
                        vt.update_field(&column_name_list[i as usize], crate::Value::NULL);
                    }
                    sqlite_lib::SQLITE_INTEGER => {
                        let value = sqlite_lib::sqlite3_column_int64(self.stmt, i);
                        vt.update_field(
                            &column_name_list[i as usize],
                            crate::Value::Integer(value),
                        );
                    }
                    sqlite_lib::SQLITE_FLOAT => {
                        let value = sqlite_lib::sqlite3_column_double(self.stmt, i);
                        // row.update_value(i as usize, crate::Value::Real(value));
                        vt.update_field(&column_name_list[i as usize], crate::Value::Real(value));
                    }
                    sqlite_lib::SQLITE_TEXT => {
                        let text_ptr = sqlite_lib::sqlite3_column_text(self.stmt, i);
                        let text_len = sqlite_lib::sqlite3_column_bytes(self.stmt, i);
                        let u8_ptr = std::slice::from_raw_parts(text_ptr, text_len as usize);
                        let str = String::from_utf8_lossy(u8_ptr).to_string();
                        // row.update_value(i as usize, crate::Value::Text(str));
                        vt.update_field(&column_name_list[i as usize], crate::Value::Text(str));
                    }
                    _ => {
                        return Err(crate::Error::InvalidColumnType(i));
                    }
                }
            }
        }
        Ok(Some(vt))
    }

    pub fn get_all_rows<T>(&self) -> Result<Vec<T>, crate::Error>
    where
        T: crate::Row,
    {
        let mut results = Vec::new();
        loop {
            let result = self.step()?;
            if result.is_none() {
                break;
            }
            let result = self.get_one_row()?;
            if result.is_none() {
                break;
            }
            results.push(result.unwrap());
        }
        Ok(results)
    }
}
