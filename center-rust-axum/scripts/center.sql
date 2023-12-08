-- label
drop table if exists label;
create table if not exists label
(
    internal_id INTEGER not null primary key autoincrement,
    id          TEXT    not null,
    parent_id   TEXT    not null,
    name        TEXT    not null,
    extra       TEXT    not null,
    create_time TEXT    not null,
    update_time TEXT    not null
);
drop index if exists unique_id;
create unique index unique_id on label (id);
drop index if exists parent_id;
create index parent_id on label (parent_id);
drop index if exists name;
create unique index name on label (name);

-- piece
drop table if exists piece;
create table if not exists piece
(
    internal_id    INTEGER not null primary key autoincrement,
    id             TEXT    not null,
    piece_type_id  TEXT    not null,
    content        TEXT    not null,
    create_time    TEXT    not null,
    update_time    TEXT    not null
);
drop index if exists unique_id;
create unique index unique_id on piece (id);
drop index if exists piece_type_id;
create index piece_type_id on piece (piece_type_id);

-- piece relation
drop table if exists piece_rel;
create table if not exists piece_rel
(
    internal_id INTEGER not null primary key autoincrement,
    parent_id   TEXT    not null,
    sub_id      TEXT    not null,
    create_time TEXT    not null,
    update_time TEXT    not null
);
drop index if exists unique_parent_sub;
create unique index unique_parent_sub on piece_rel (parent_id, sub_id);
drop index if exists parent_id;
create index parent_id on piece_rel (parent_id);
drop index if exists sub_id;
create index sub_id on piece_rel (sub_id);

-- piece label relation
drop table if exists piece_label_rel;
create table if not exists piece_label_rel
(
    internal_id INTEGER not null primary key autoincrement,
    piece_id    TEXT    not null,
    label_id    TEXT    not null,
    create_time TEXT    not null
);
drop index if exists unique_piece_label;
create unique index unique_piece_label on piece_label_rel (piece_id, label_id);
drop index if exists label_id;
create unique index label_id on piece_label_rel (label_id);
drop index if exists piece_id;
create unique index piece_id on piece_label_rel (piece_id);





-- piece_type
drop table if exists piece_type;
create table if not exists piece_type
(
	internal_id INTEGER not null primary key autoincrement,
	id          TEXT    not null,
	name        TEXT    not null,
	description TEXT    not null,
	create_time TEXT    not null,
	update_time TEXT    not null
);
drop index if exists unique_id;
create unique index unique_id on piece_type (id);
drop index if exists unique_name;
create unique index unique_name on piece_type (name);
