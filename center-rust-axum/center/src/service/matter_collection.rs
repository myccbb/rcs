pub const MATTER_COLLECTION_PIECE_TYPE_ID: &str = "matter_collection";

#[derive(serde::Serialize, serde::Deserialize, Clone, Default, Debug, PartialEq)]
pub struct MatterCollection {
    pub create_time: chrono::DateTime<chrono::FixedOffset>,
    pub update_time: chrono::DateTime<chrono::FixedOffset>,

    pub id: String,
    pub content: MatterCollectionContent,
}

use anyhow::Context;

impl MatterCollection {
    pub fn new(id: String, title: String) -> Self {
        Self {
            id,
            content: MatterCollectionContent::new(title),
            ..Default::default()
        }
    }

    pub async fn get_by_id(
        sqlite_conn: &sqlite::Connection,
        id: &str,
    ) -> anyhow::Result<Option<Self>> {
        let result = crate::db::Piece::get_by_id(sqlite_conn, id);
        if let Err(e) = result {
            if e == crate::db::Error::RecordNotFound {
                return Ok(None);
            }
            return Err(anyhow::anyhow!("get matter collection piece failed"));
        }
        let piece = result.unwrap();
        if piece.is_none() {
            return Ok(None);
        }
        let mc = MatterCollection::from_db_piece(&piece.unwrap())?;
        Ok(Some(mc))
    }

    pub fn from_db_piece(piece: &crate::db::Piece) -> Result<Self, crate::service::Error> {
        let mut content = piece.content.as_str();
        if content.is_empty() {
            content = "{}";
        }
        Ok(MatterCollection {
            create_time: piece.create_time,
            update_time: piece.update_time,
            id: piece.id.clone(),
            content: serde_json::from_str(content)?,
        })
    }

    pub fn to_db_piece(&self) -> Result<crate::db::Piece, crate::service::Error> {
        let content = serde_json::to_string(&self.content)?;
        let piece = crate::db::Piece {
            id: self.id.clone(),
            piece_type_id: MATTER_COLLECTION_PIECE_TYPE_ID.to_string(),
            content,
            ..Default::default()
        };
        Ok(piece)
    }

    pub async fn move_matter_collection(
        sqlite_conn: &sqlite::Connection,
        src_parent_mc_id: &str,
        src_mc_id: &str,
        dest_parent_mc_id: &str,
        dest_pos: usize,
    ) -> anyhow::Result<()> {
        let mut src_parent_mc =
            crate::service::MatterCollection::get_by_id(sqlite_conn, src_parent_mc_id)
                .await
                .context("failed to get src parent matter collection")?
                .ok_or(anyhow::anyhow!("src parent matter collection not found"))?;

        let src_mc_ref = src_parent_mc
            ._remove_matter_collection(src_mc_id)
            .ok_or(anyhow::anyhow!("src matter collection not found"))?;

        let mut dest_parent_mc =
            crate::service::MatterCollection::get_by_id(sqlite_conn, dest_parent_mc_id)
                .await
                .context("failed to get dest parent matter collection")?
                .ok_or(anyhow::anyhow!("dest parent matter collection not found"))?;

        dest_parent_mc._insert_matter_collection(src_mc_ref, dest_pos);

        if src_parent_mc_id != dest_parent_mc_id {
            crate::db::PieceRel::new(
                src_parent_mc_id,
                src_mc_id,
                crate::db::PieceRelType::MatterCollection,
            )
            .update_parent_id(sqlite_conn, dest_parent_mc_id)
            .await?;
        }
        Ok(())
    }

    fn _remove_matter_collection(&mut self, mc_id: &str) -> Option<MatterCollectionRef> {
        let mut index = None;
        for (i, mc) in self.content.sub_matter_collection_list.iter().enumerate() {
            if mc.id == mc_id {
                index = Some(i);
                break;
            }
        }
        if let Some(i) = index {
            return Some(self.content.sub_matter_collection_list.remove(i));
        }
        None
    }

    fn _insert_matter_collection(&mut self, mc_ref: MatterCollectionRef, pos: usize) {
        if pos > self.content.sub_matter_collection_list.len() {
            self.content.sub_matter_collection_list.push(mc_ref.clone());
        }
        self.content
            .sub_matter_collection_list
            .insert(pos, mc_ref.clone());
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Default, Debug, PartialEq)]
pub struct MatterCollectionContent {
    pub title: String,
    pub matter_list: Vec<crate::service::MatterItemRef>,
    pub sub_matter_collection_list: Vec<MatterCollectionRef>,
}

impl MatterCollectionContent {
    pub fn new(title: String) -> Self {
        Self {
            title,
            ..Default::default()
        }
    }
}

pub struct MatterCollectionRefList(Vec<MatterCollectionRef>);

impl MatterCollectionRefList {
    pub async fn load_all(&mut self, _conn: &sqlite::Connection) -> anyhow::Result<()> {
        for _mc_ref in self.0.iter_mut() {
            // mc_ref.load_all(&mut *conn).await?;
        }
        Ok(())
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Default, Debug, PartialEq)]
pub struct MatterCollectionRef {
    pub id: String,

    pub sub_matter_collection_list: Vec<MatterCollectionRef>,
}

impl std::convert::From<MatterCollection> for MatterCollectionRef {
    fn from(mc: MatterCollection) -> Self {
        Self {
            id: mc.id,
            sub_matter_collection_list: mc.content.sub_matter_collection_list,
        }
    }
}

impl MatterCollectionRef {
    pub fn new(matter_collection_id: String) -> Self {
        Self {
            id: matter_collection_id,
            ..Default::default()
        }
    }

    pub fn find_by_id(&mut self, id: &str) -> Option<&mut MatterCollectionRef> {
        if self.id == id {
            return Some(self);
        }
        for mc_ref in self.sub_matter_collection_list.iter_mut() {
            let mc_ref = mc_ref.find_by_id(id);
            if mc_ref.is_some() {
                return mc_ref;
            }
        }
        None
    }

    pub async fn load_all_non_recursive(
        &mut self,
        conn: &sqlite::Connection,
    ) -> anyhow::Result<()> {
        let mut mc_id_queue: std::collections::VecDeque<String> = std::collections::VecDeque::new();
        for sub_mc_ref in self.sub_matter_collection_list.iter() {
            mc_id_queue.push_back(sub_mc_ref.id.clone());
        }
        loop {
            let mc_id = mc_id_queue.pop_front();
            if mc_id.is_none() {
                break;
            }
            let mc_id = mc_id.unwrap();
            let mc_obj = crate::service::MatterCollection::get_by_id(conn, &mc_id)
                .await?
                .ok_or(anyhow::anyhow!("matter collection not found"))?;
            let mc_ref = self.find_by_id(&mc_id).unwrap();
            for sub_mc_ref in mc_obj.content.sub_matter_collection_list.iter() {
                mc_id_queue.push_back(sub_mc_ref.id.clone());
                mc_ref
                    .sub_matter_collection_list
                    .push(crate::service::MatterCollectionRef::new(
                        sub_mc_ref.id.clone(),
                    ))
            }
        }
        Ok(())
    }

    pub fn from_db_piece(piece: &crate::db::Piece) -> Result<Self, crate::service::Error> {
        let mut content = piece.content.as_str();
        if content.is_empty() {
            content = "{}";
        }
        Ok(MatterCollectionRef {
            id: piece.id.clone(),
            sub_matter_collection_list: serde_json::from_str(content)?,
        })
    }
}
