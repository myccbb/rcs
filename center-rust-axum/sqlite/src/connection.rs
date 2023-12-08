use libsqlite3_sys as sqlite_lib;

#[derive(Copy, Clone, Debug)]
pub enum OpenMode {
    Memory,
    File,
}

// struct SqliteConn(sqlite_lib::sqlite3);

#[derive(Debug, Clone)]
pub struct Connection {
    is_open: bool,
    mode: OpenMode,
    db_name: String,
    conn: *mut sqlite_lib::sqlite3,
}

unsafe impl Send for Connection {}
unsafe impl Sync for Connection {}

use crate::Executor;

impl Connection {
    pub fn new() -> Self {
        Self {
            mode: OpenMode::Memory,
            db_name: ":memory:".into(),
            conn: std::ptr::null_mut(),
            ..Default::default()
        }
    }
    pub fn close(&mut self) -> &mut Self {
        unsafe {
            sqlite_lib::sqlite3_close_v2(self.conn);
        }
        self.conn = std::ptr::null_mut();
        self.is_open = false;
        self
    }
    pub fn db_name(&self) -> String {
        self.db_name.clone()
    }
    pub fn mode(&self) -> OpenMode {
        self.mode
    }
    pub fn open_file(filename: &str) -> Result<Self, crate::Error> {
        let mut c = Self {
            mode: OpenMode::File,
            db_name: filename.into(),
            is_open: false,
            conn: std::ptr::null_mut(),
        };

        c.conn = unsafe {
            let mut db: *mut sqlite_lib::sqlite3 = std::ptr::null_mut();
            let filename = std::ffi::CString::new(filename)?;
            let res_code = sqlite_lib::sqlite3_open_v2(
                filename.as_ptr(),
                &mut db,
                sqlite_lib::SQLITE_OPEN_READWRITE
                    | sqlite_lib::SQLITE_OPEN_CREATE
                    | sqlite_lib::SQLITE_OPEN_FULLMUTEX,
                std::ptr::null(),
            );
            if res_code != sqlite_lib::SQLITE_OK {
                let mut err_detail = None;
                if !db.is_null() {
                    let c_slice =
                        std::ffi::CStr::from_ptr(sqlite_lib::sqlite3_errmsg(db)).to_bytes();
                    err_detail = Some(String::from_utf8_lossy(c_slice).into_owned());
                }
                return Err(crate::Error::OpenConnectionFail(
                    res_code,
                    sqlite_lib::code_to_str(res_code).into(),
                    err_detail,
                ));
            }
            let _ = sqlite_lib::sqlite3_extended_result_codes(db, 1);
            db
        };
        c.is_open = true;
        Ok(c)
    }
    pub fn open_memory(db_name: &str) -> Result<Self, crate::Error> {
        let mut c = Self {
            mode: OpenMode::Memory,
            db_name: db_name.into(),
            is_open: false,
            conn: std::ptr::null_mut(),
        };

        c.conn = unsafe {
            let mut db: *mut sqlite_lib::sqlite3 = std::ptr::null_mut();
            let res_code = sqlite_lib::sqlite3_open_v2(
                db_name.as_ptr() as *const std::os::raw::c_char,
                &mut db,
                sqlite_lib::SQLITE_OPEN_READWRITE
                    | sqlite_lib::SQLITE_OPEN_CREATE
                    | sqlite_lib::SQLITE_OPEN_FULLMUTEX
                    | sqlite_lib::SQLITE_OPEN_MEMORY,
                std::ptr::null(),
            );
            if res_code != sqlite_lib::SQLITE_OK {
                let mut err_detail = None;
                if !db.is_null() {
                    let c_slice =
                        std::ffi::CStr::from_ptr(sqlite_lib::sqlite3_errmsg(db)).to_bytes();
                    err_detail = Some(String::from_utf8_lossy(c_slice).into_owned());
                }
                return Err(crate::Error::OpenConnectionFail(
                    res_code,
                    sqlite_lib::code_to_str(res_code).into(),
                    err_detail,
                ));
            }
            let _ = sqlite_lib::sqlite3_extended_result_codes(db, 1);
            db
        };
        c.is_open = true;
        Ok(c)
    }

    pub fn prepare(&self, sql: &str) -> Result<crate::Statement, crate::Error> {
        let mut stmt: *mut sqlite_lib::sqlite3_stmt = std::ptr::null_mut();
        unsafe {
            let res_code = sqlite_lib::sqlite3_prepare_v2(
                self.conn,
                std::ffi::CString::new(sql)?.as_ptr(),
                sql.len() as i32,
                &mut stmt,
                std::ptr::null_mut(),
            );
            if res_code != sqlite_lib::SQLITE_OK {
                return Err(crate::Error::PrepareFail(
                    res_code,
                    sqlite_lib::code_to_str(res_code).into(),
                ));
            }
        }
        Ok(crate::Statement::new(stmt))
    }
    pub fn fetch_all_raw_rows(
        &self,
        sql: &str,
        params: Option<Vec<crate::Value>>,
    ) -> Result<Vec<crate::RawRow>, crate::Error> {
        let mut stmt = self.prepare(sql)?;
        if let Some(params) = params {
            stmt.bind(params)?;
        }
        let rows = stmt.get_all_raw_rows()?;
        stmt.finalize();
        Ok(rows)
    }
    pub fn begin_transaction(&self) -> Result<crate::Transaction, crate::Error> {
        self.execute("begin transaction;", None)?;
        Ok(crate::Transaction::new(self.clone()))
    }
}

impl crate::Executor for Connection {
    fn execute(&self, sql: &str, params: Option<Vec<crate::Value>>) -> Result<(), crate::Error> {
        let mut stmt = self.prepare(sql)?;
        if let Some(params) = params {
            stmt.bind(params)?;
        }
        stmt.step()?;
        stmt.finalize();
        Ok(())
    }
    fn fetch_all_rows<T>(
        &self,
        sql: &str,
        params: Option<Vec<crate::Value>>,
    ) -> Result<Vec<T>, crate::Error>
    where
        T: crate::Row,
    {
        let mut stmt = self.prepare(sql)?;
        if let Some(params) = params {
            stmt.bind(params)?;
        }
        let rows = stmt.get_all_rows()?;
        stmt.finalize();
        Ok(rows)
    }
}

impl Default for Connection {
    fn default() -> Self {
        Self::new()
    }
}
