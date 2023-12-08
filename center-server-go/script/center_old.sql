-- label
drop table if exists label;
create table if not exists label
(
    internal_id INTEGER not null primary key autoincrement,
    id          TEXT    not null,
    name        TEXT    not null,
    parent_id   TEXT    not null,
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

-- object
drop table if exists object;
create table if not exists object
(
    internal_id    INTEGER not null primary key autoincrement,
    id             TEXT    not null,
    object_type_id TEXT    not null,
    title          TEXT    not null default '',
    content        TEXT    not null,
    create_time    TEXT    not null,
    update_time    TEXT    not null
);
drop index if exists unique_id;
create unique index unique_id on object (id);
drop index if exists object_type_id;
create index object_type_id on object (object_type_id);

-- object relation
drop table if exists object_rel;
create table if not exists object_rel
(
    internal_id INTEGER not null primary key autoincrement,
    parent_id   TEXT    not null,
    sub_id      TEXT    not null,
    create_time TEXT    not null,
    update_time TEXT    not null
);
drop index if exists unique_parent_sub;
create unique index unique_parent_sub on object_rel (parent_id, sub_id);
drop index if exists parent_id;
create index parent_id on object_rel (parent_id);
drop index if exists sub_id;
create index sub_id on object_rel (sub_id);

-- object label relation
drop table if exists object_label_rel;
create table if not exists object_label_rel
(
    internal_id INTEGER not null primary key autoincrement,
    object_id    TEXT    not null,
    label_id    TEXT    not null,
    create_time TEXT    not null
);
drop index if exists unique_object_label;
create unique index unique_object_label on object_label_rel (object_id, label_id);
drop index if exists label_id;
create unique index label_id on object_label_rel (label_id);
drop index if exists object_id;
create unique index object_id on object_label_rel (object_id);

-- collection
drop table if exists collection;
create table if not exists collection
(
    internal_id    INTEGER not null primary key autoincrement,
    id             TEXT    not null,
    object_type_id TEXT    not null,
    title          TEXT    not null default '',
    content        TEXT    not null,
    create_time    TEXT    not null,
    update_time    TEXT    not null
);
drop index if exists unique_id;
create unique index unique_id on collection (id);
drop index if exists object_type_id;
create index object_type_id on collection (object_type_id);

-- collection relation
drop table if exists collection_rel;
create table if not exists collection_rel
(
    internal_id INTEGER not null primary key autoincrement,
    parent_id   TEXT    not null,
    sub_id      TEXT    not null,
    create_time TEXT    not null,
    update_time TEXT    not null
);
drop index if exists unique_parent_sub;
create unique index unique_parent_sub on collection_rel (parent_id, sub_id);
drop index if exists parent_id;
create index parent_id on collection_rel (parent_id);
drop index if exists sub_id;
create index sub_id on collection_rel (sub_id);


-- object collection relation
drop table if exists collection_object_rel;
create table if not exists collection_object_rel
(
    internal_id   INTEGER not null primary key autoincrement,
    collection_id TEXT    not null,
    object_id      TEXT    not null,
    create_time   TEXT    not null,
    update_time   TEXT    not null
);
drop index if exists unique_collection_object;
create unique index unique_collection_object on collection_object_rel (collection_id, object_id);
drop index if exists collection_id;
create index collection_id on collection_object_rel (collection_id);
drop index if exists object_id;
create index object_id on collection_object_rel (object_id);

-- collection label relation
drop table if exists collection_label_rel;
create table if not exists collection_label_rel
(
    internal_id   INTEGER not null primary key autoincrement,
    collection_id TEXT    not null,
    label_id      TEXT    not null,
    create_time   TEXT    not null
);
drop index if exists unique_collection_label;
create unique index unique_collection_label on collection_label_rel (collection_id, label_id);
drop index if exists label_id;
create index label_id on collection_label_rel (label_id);
drop index if exists collection_id;
create index collection_id on collection_label_rel (collection_id);


-- object_type
drop table if exists object_type;
create table if not exists object_type
(
    internal_id INTEGER not null primary key autoincrement,
    id          TEXT    not null,
    name        TEXT    not null,
    category    TEXT    not null,
    description TEXT    not null,
    create_time TEXT    not null,
    update_time TEXT    not null
);
drop index if exists unique_id;
create unique index unique_id on object_type (id);
drop index if exists unique_category_name;
create unique index unique_category_name on object_type (category, name);
