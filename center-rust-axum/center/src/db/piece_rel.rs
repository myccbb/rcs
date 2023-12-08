use sqlite::Executor;

#[derive(
    sqlite_macros::Row, serde::Serialize, serde::Deserialize, Clone, Default, Debug, PartialEq,
)]
pub struct PieceRel {
    pub internal_id: i64,
    pub create_time: chrono::DateTime<chrono::FixedOffset>,
    pub update_time: chrono::DateTime<chrono::FixedOffset>,
    pub parent_id: String,
    pub sub_id: String,
    pub rel_type: String,
}

impl PieceRel {
    pub fn new(parent_id: &str, sub_id: &str, rel_type: PieceRelType) -> Self {
        Self {
            parent_id: parent_id.to_string(),
            sub_id: sub_id.to_string(),
            rel_type: rel_type.to_string(),
            ..Default::default()
        }
    }

    pub fn get(&mut self, conn: &sqlite::Connection) -> Result<(), crate::db::Error> {
        let sql = "SELECT * FROM piece_rel WHERE rel_type = ? and parent_id = ? AND sub_id = ?;";
        conn.execute(
            sql,
            Some(vec![
                self.rel_type.clone().into(),
                self.parent_id.clone().into(),
                self.sub_id.clone().into(),
            ]),
        )?;
        Ok(())
    }

    pub fn create(&mut self, conn: &sqlite::Connection) -> Result<(), crate::db::Error> {
        let now = crate::dist::utils::now_beijing();
        if self.create_time.timestamp() == 0 {
            self.create_time = now;
        }
        if self.update_time.timestamp() == 0 {
            self.update_time = now;
        }
        let sql = "INSERT OR IGNORE INTO \
            piece_rel(create_time, update_time, parent_id, sub_id) \
            VALUES(?,?,?,?)";
        conn.execute(
            sql,
            Some(vec![
                self.create_time.into(),
                self.update_time.into(),
                self.parent_id.clone().into(),
                self.sub_id.clone().into(),
            ]),
        )?;
        Ok(())
    }

    pub async fn update_parent_id(
        &self,
        conn: &sqlite::Connection,
        new_parent_id: &str,
    ) -> Result<(), crate::db::Error> {
        let now = crate::dist::utils::now_beijing();
        let sql = "UPDATE piece_rel SET parent_id = ? AND update_time = ? \
        WHERE parent_id = ? AND sub_id = ?";
        conn.execute(
            sql,
            Some(vec![
                new_parent_id.into(),
                now.into(),
                self.parent_id.clone().into(),
                self.sub_id.clone().into(),
            ]),
        )?;
        Ok(())
    }
}

pub enum PieceRelType {
    DailyMatterCollection,
    DailyMatterItem,
    MatterCollectionItem,
    MatterCollection,
    MatterItem,
}

impl std::string::ToString for PieceRelType {
    fn to_string(&self) -> String {
        match self {
            PieceRelType::DailyMatterCollection => "daily_matter_collection".to_string(),
            PieceRelType::DailyMatterItem => "daily_matter_item".to_string(),
            PieceRelType::MatterCollectionItem => "matter_collection_item".to_string(),
            PieceRelType::MatterCollection => "matter_collection".to_string(),
            PieceRelType::MatterItem => "matter_item".to_string(),
        }
    }
}
