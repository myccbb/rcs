use crate::db::Error;
use sqlite::Executor;

#[derive(sqlite_macros::Row, serde::Serialize, serde::Deserialize, Clone, Default, Debug)]
pub struct Piece {
    pub internal_id: i64,
    pub create_time: chrono::DateTime<chrono::FixedOffset>,
    pub update_time: chrono::DateTime<chrono::FixedOffset>,
    pub id: String,
    pub piece_type_id: String,
    pub content: String,
}

impl Piece {
    pub fn new(id: String) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    pub fn create<T>(&mut self, conn: &T) -> Result<(), Error>
    where
        T: sqlite::Executor,
    {
        let now = crate::dist::utils::now_beijing();
        if self.create_time.timestamp() == 0 {
            self.create_time = now;
        }
        if self.update_time.timestamp() == 0 {
            self.update_time = now;
        }
        if self.content.is_empty() {
            self.content = "{}".to_string();
        }
        let sql = "INSERT OR IGNORE INTO piece\
    (create_time, update_time, id, piece_type_id, content) \
    VALUES(?,?,?,?,?)";
        conn.execute(
            sql,
            Some(vec![
                self.create_time.into(),
                self.update_time.into(),
                self.id.clone().into(),
                self.piece_type_id.clone().into(),
                self.content.clone().into(),
            ]),
        )?;
        Ok(())
    }

    pub fn get_by_id(conn: &sqlite::Connection, id: &str) -> Result<Option<Self>, Error> {
        let sql = "SELECT * FROM piece WHERE id = ?;";
        let results = conn.fetch_all_rows::<Self>(sql, Some(vec![id.into()]))?;
        if results.is_empty() {
            return Ok(None);
        }
        Ok(Some(results[0].clone()))
    }

    pub fn raw_delete(&self, conn: &sqlite::Connection) -> Result<(), Error> {
        let sql = "DELETE FROM piece WHERE id=?;";
        conn.execute(sql, Some(vec![self.id.clone().into()]))?;
        Ok(())
    }

    pub async fn update_by_id(&self, conn: &sqlite::Connection) -> Result<(), Error> {
        let sql = "UPDATE piece SET content=?, update_time=? WHERE id=?;";
        conn.execute(
            sql,
            Some(vec![
                self.content.clone().into(),
                self.update_time.into(),
                self.id.clone().into(),
            ]),
        )?;
        Ok(())
    }
}

impl std::cmp::PartialEq for Piece {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.piece_type_id == other.piece_type_id
            && self.content == other.content
            && self.create_time == other.create_time
            && self.update_time == other.update_time
    }
}
