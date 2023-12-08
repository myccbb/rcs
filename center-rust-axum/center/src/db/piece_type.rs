use sqlite::Executor;

use crate::db::error;

#[derive(sqlite_macros::Row, serde::Serialize, serde::Deserialize, Clone, Default, Debug)]
pub struct PieceType {
    pub internal_id: i64,
    pub create_time: chrono::DateTime<chrono::FixedOffset>,
    pub update_time: chrono::DateTime<chrono::FixedOffset>,
    pub id: String,
    pub name: String,
    pub description: String,
}

impl PieceType {
    pub fn new(id: String) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    pub fn create_sync<T>(&mut self, conn: &T) -> Result<(), error::Error>
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
        let sql =
            "INSERT OR IGNORE INTO piece_type(create_time, update_time, id, name, description) \
            VALUES(?,?,?,?,?)";
        let result = conn.execute(
            sql,
            Some(vec![
                self.create_time.into(),
                self.update_time.into(),
                self.id.clone().into(),
                self.name.clone().into(),
                self.description.clone().into(),
            ]),
        );
        // println!("{}", self.create_time.to_rfc3339());
        if let Err(e) = result {
            return Err(error::Error::from_sqlite(&e));
        }
        Ok(())
    }

    pub fn list_all_sync(
        conn: &sqlite::Connection,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<PieceType>, sqlite::Error> {
        let sql = "SELECT * FROM piece_type LIMIT ? OFFSET ?";
        let result = conn.fetch_all_rows::<PieceType>(
            sql,
            Some(vec![page_size.into(), ((page - 1) * page_size).into()]),
        )?;
        Ok(result)
    }

    pub fn list_all(
        conn: &sqlite::Connection,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<PieceType>, sqlite::Error> {
        let sql = "SELECT * FROM piece_type LIMIT ? OFFSET ?";
        let results = conn.fetch_all_rows::<PieceType>(
            sql,
            Some(vec![page_size.into(), ((page - 1) * page_size).into()]),
        )?;
        Ok(results)
    }
    pub async fn get_by_name(
        conn: &sqlite::Connection,
        name: &str,
    ) -> Result<Option<PieceType>, crate::db::Error> {
        let sql = "SELECT * FROM piece_type WHERE name=?";
        let piece_type = conn.fetch_all_rows::<PieceType>(sql, Some(vec![name.into()]))?;
        if piece_type.is_empty() {
            return Ok(None);
        }
        Ok(Some(piece_type[0].clone()))
    }
    pub async fn get_by_id(
        conn: &sqlite::Connection,
        id: &str,
    ) -> Result<Option<PieceType>, sqlite::Error> {
        let sql = "SELECT * FROM piece_type WHERE id=?";
        let piece_type = conn.fetch_all_rows::<PieceType>(sql, Some(vec![id.into()]))?;
        if piece_type.is_empty() {
            return Ok(None);
        }
        Ok(Some(piece_type[0].clone()))
    }
    pub fn update_by_id(
        &self,
        conn: &sqlite::Connection,
        id: &str,
    ) -> Result<(), crate::db::Error> {
        let sql = "UPDATE piece_type SET name=?, description=?, update_time=? WHERE id=?";
        conn.execute(
            sql,
            Some(vec![
                self.name.clone().into(),
                self.description.clone().into(),
                self.update_time.into(),
                id.into(),
            ]),
        )?;
        Ok(())
    }
    pub fn to_short_string(&self) -> String {
        format!(
            "{}, {}, {}, {}, {}, {}",
            &self.internal_id,
            &self.id,
            &self.name,
            &self.description,
            &self.create_time.to_rfc3339(),
            &self.update_time.to_rfc3339(),
        )
    }
    pub fn to_detail_string(&self) -> String {
        format!(
            "internal_id: {},id: {}, name: {}, desc: {}, create_time: {}, update_time:{}",
            &self.internal_id,
            &self.id,
            &self.name,
            &self.description,
            &self.create_time.to_rfc3339(),
            &self.update_time.to_rfc3339(),
        )
    }
}

impl std::fmt::Display for PieceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_short_string())
    }
}

impl PartialEq for PieceType {
    fn eq(&self, other: &Self) -> bool {
        self.internal_id == other.internal_id
            && self.create_time == other.create_time
            && self.update_time == other.update_time
            && self.id == other.id
            && self.name == other.name
            && self.description == other.description
    }
}
