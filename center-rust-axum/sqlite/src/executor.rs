pub trait Executor {
    fn execute(&self, sql: &str, params: Option<Vec<crate::Value>>) -> Result<(), crate::Error>;
    fn fetch_all_rows<T>(
        &self,
        sql: &str,
        params: Option<Vec<crate::Value>>,
    ) -> Result<Vec<T>, crate::Error>
    where
        T: crate::Row;
}
