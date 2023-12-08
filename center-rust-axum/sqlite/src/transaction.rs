pub struct Transaction {
    conn: crate::Connection,
}

use crate::Executor;
impl Drop for Transaction {
    fn drop(&mut self) {
        let _ = self.conn.execute("ROLLBACK", None);
    }
}

impl Transaction {
    pub fn new(conn: crate::Connection) -> Self {
        Transaction { conn }
    }
    pub fn commit(&self) -> Result<(), crate::Error> {
        self.conn.execute("COMMIT", None)
    }
    pub fn rollback(&self) -> Result<(), crate::Error> {
        self.conn.execute("ROLLBACK", None)
    }
}

impl crate::Executor for Transaction {
    fn execute(&self, sql: &str, params: Option<Vec<crate::Value>>) -> Result<(), crate::Error> {
        self.conn.execute(sql, params)
    }
    fn fetch_all_rows<T>(
        &self,
        sql: &str,
        params: Option<Vec<crate::Value>>,
    ) -> Result<Vec<T>, crate::Error>
    where
        T: crate::Row,
    {
        self.conn.fetch_all_rows(sql, params)
    }
}
