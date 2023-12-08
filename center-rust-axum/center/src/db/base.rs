use log::info;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct BaseModel {
    pub internal_id: i64,
    pub create_time: String,
    pub update_time: String,
}

const INIT_TABLE_SQL: &[&str] = &[
"create table if not exists label
(
	internal_id INTEGER not null primary key autoincrement,
	id          TEXT    not null,
	name        TEXT    not null,
	parent_id   TEXT    not null,
	extra       TEXT    not null,
	create_time TEXT    not null,
	update_time TEXT    not null
);",
"create unique index if not exists label_unique_id on label (id);",
"create index if not exists label_parent_id on label (parent_id);",
"create unique index if not exists label_name on label (name);",
"create table if not exists piece
(
	internal_id    INTEGER not null primary key autoincrement,
	id             TEXT    not null,
	piece_type_id  TEXT    not null,
	content        TEXT    not null,
	create_time    TEXT    not null,
	update_time    TEXT    not null
);",
"create unique index if not exists piece_unique_id on piece (id);",
"create index if not exists piece_piece_type_id on piece (piece_type_id);",
"create table if not exists piece_rel
(
	internal_id INTEGER not null primary key autoincrement,
	parent_id   TEXT    not null,
	sub_id      TEXT    not null,
	create_time TEXT    not null,
	update_time TEXT    not null
);",
"create unique index if not exists piece_rel_unique_parent_sub on piece_rel (parent_id, sub_id);",
"create index if not exists piece_rel_parent_id on piece_rel (parent_id);",
"create index if not exists piece_rel_sub_id on piece_rel (sub_id);",
"create table if not exists piece_label_rel
(
	internal_id INTEGER not null primary key autoincrement,
	piece_id    TEXT    not null,
	label_id    TEXT    not null,
	create_time TEXT    not null
);",
"create unique index if not exists piece_label_rel_unique_piece_label on piece_label_rel (piece_id, label_id);",
"create unique index if not exists piece_label_rel_label_id on piece_label_rel (label_id);",
"create unique index if not exists piece_label_rel_piece_id on piece_label_rel (piece_id);",
"create table if not exists piece_type
(
	internal_id INTEGER not null primary key autoincrement,
	id          TEXT    not null,
	name        TEXT    not null,
	description TEXT    not null,
	create_time TEXT    not null,
	update_time TEXT    not null
);",
"create unique index if not exists piece_type_unique_id on piece_type (id);",
"create unique index if not exists piece_type_unique_name on piece_type (name);",
];

pub fn init_tables_sync(conn: &sqlite::Connection) -> Result<(), sqlite::Error> {
    let re_newline: Regex = Regex::new(r"\n+").unwrap();
    let re_whitespace: Regex = Regex::new(r"\s+").unwrap();

    use sqlite::Executor;
    for sql in INIT_TABLE_SQL {
        let sql = re_newline.replace_all(sql, "");
        let sql = re_whitespace.replace_all(&sql, " ");
        info!("{}", sql);
        conn.execute(&sql, None)?;
    }
    Ok(())
}
