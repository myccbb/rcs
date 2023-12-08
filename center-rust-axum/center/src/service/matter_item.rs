pub const MATTER_PIECE_TYPE_ID: &str = "matter";

#[derive(serde::Serialize, serde::Deserialize, Clone, Default, Debug, PartialEq)]
pub struct MatterItem {
    internal_id: i64,
    create_time: chrono::DateTime<chrono::FixedOffset>,
    update_time: chrono::DateTime<chrono::FixedOffset>,

    id: String,
    piece_type_id: String,
    content: MatterItemContent,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Default, Debug, PartialEq)]
pub struct MatterItemContent {
    title: String,
    sub_matter_list: Vec<MatterItemRef>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Default, Debug, PartialEq)]
pub struct MatterItemRef {
    create_time: chrono::DateTime<chrono::FixedOffset>,
    update_time: chrono::DateTime<chrono::FixedOffset>,
    matter_id: String,

    sub_matter_list: Vec<MatterItemRef>,
}
