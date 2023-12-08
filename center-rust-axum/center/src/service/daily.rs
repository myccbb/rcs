use anyhow::Context;

const DAILY_PIECE_ID: &str = "daily";
const DAILY_PIECE_TYPE_ID: &str = "daily";

#[derive(serde::Serialize, serde::Deserialize, Clone, Default, Debug, PartialEq)]
pub struct Daily {
    id: String,
    content: DailyContent,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Default, Debug, PartialEq)]
struct DailyContent {
    default_matter_collection: crate::service::MatterCollectionRef,
}

impl Daily {
    pub async fn new(conn: &sqlite::Connection) -> anyhow::Result<Option<Daily>> {
        let result = crate::db::Piece::get_by_id(conn, DAILY_PIECE_ID);
        if let Err(e) = result {
            log::error!("error: {}", e.to_string());
            if e == crate::db::Error::RecordNotFound {
                return Ok(None);
            }
            return Err(anyhow::anyhow!("get daily piece failed"));
        }
        let piece = result?;
        if piece.is_none() {
            return Ok(None);
        }
        let piece = piece.unwrap();
        let daily = Daily {
            id: piece.id,
            content: serde_json::from_str(&piece.content).context("parse daily content failed")?,
        };
        Ok(Some(daily))
    }

    pub async fn load_all(&mut self, _conn: &sqlite::Connection) -> anyhow::Result<()> {
        Ok(())
    }

    pub async fn create_matter_collection(
        &mut self,
        sqlite_conn: &sqlite::Connection,
        title: String,
    ) -> Result<Option<crate::service::MatterCollectionRef>, crate::service::Error> {
        let mc_id = crate::service::random_piece_id();

        let mc = crate::service::MatterCollection::new(mc_id.clone(), title.clone());

        let mut mc_piece = mc.to_db_piece()?;
        mc_piece.create(sqlite_conn);

        self.content
            .default_matter_collection
            .sub_matter_collection_list
            .push(mc.into());

        let mut daily_mc_rel = crate::db::PieceRel::new(
            DAILY_PIECE_ID,
            &mc_id,
            crate::db::PieceRelType::MatterCollection,
        );
        daily_mc_rel.create(sqlite_conn);
        Ok(None)
    }

    pub fn remove_matter_collection(
        &mut self,
        mc_id: &str,
    ) -> Option<crate::service::MatterCollectionRef> {
        let mut index = None;
        for (i, mc) in self
            .content
            .default_matter_collection
            .sub_matter_collection_list
            .iter()
            .enumerate()
        {
            if mc.id == mc_id {
                index = Some(i);
                break;
            }
        }
        if let Some(i) = index {
            return Some(
                self.content
                    .default_matter_collection
                    .sub_matter_collection_list
                    .remove(i),
            );
        }
        None
    }

    pub async fn move_matter_collection(
        sqlite_conn: &sqlite::Connection,
        src_parent_mc_id: String,
        src_mc_id: String,
        dest_parent_mc_id: String,
        dest_pos: usize,
    ) -> anyhow::Result<()> {
        crate::service::MatterCollection::move_matter_collection(
            sqlite_conn,
            &src_parent_mc_id,
            &src_mc_id,
            &dest_parent_mc_id,
            dest_pos,
        )
        .await?;
        Ok(())
    }
}

pub fn init_daily(sqlite_conn: &sqlite::Connection) -> Result<(), crate::service::Error> {
    let tx = sqlite_conn.begin_transaction()?;
    let mut daily_piece_type = crate::db::PieceType {
        id: DAILY_PIECE_TYPE_ID.to_string(),
        name: DAILY_PIECE_TYPE_ID.to_string(),
        ..Default::default()
    };
    let result = daily_piece_type.create_sync(&tx);
    if let Err(e) = result {
        log::error!("create daily piece_type failed, {}", e.to_string());
        return Err(e.into());
    }

    let mut mc_piece_type = crate::db::PieceType {
        id: crate::service::matter_collection::MATTER_COLLECTION_PIECE_TYPE_ID.to_string(),
        name: crate::service::matter_collection::MATTER_COLLECTION_PIECE_TYPE_ID.to_string(),
        ..Default::default()
    };
    let result = mc_piece_type.create_sync(&tx);
    if let Err(e) = result {
        log::error!(
            "create matter_collection piece_type failed, {}",
            e.to_string()
        );
        return Err(e.into());
    }

    let mut matter_piece_type = crate::db::PieceType {
        id: crate::service::MATTER_PIECE_TYPE_ID.to_string(),
        name: crate::service::MATTER_PIECE_TYPE_ID.to_string(),
        ..Default::default()
    };
    let result = matter_piece_type.create_sync(&tx);
    if let Err(e) = result {
        log::error!("create matter piece_type failed, {}", e.to_string());
        return Err(e.into());
    }

    let mut daily_piece = crate::db::Piece {
        id: DAILY_PIECE_ID.to_string(),
        piece_type_id: DAILY_PIECE_TYPE_ID.to_string(),
        ..Default::default()
    };
    let result = daily_piece.create(&tx);
    if let Err(e) = result {
        log::error!("create daily piece failed, {}", e.to_string());
        return Err(e.into());
    }

    tx.commit()?;

    Ok(())
}
