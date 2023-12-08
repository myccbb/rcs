create database if not exists center character set utf8mb4 collate utf8mb4_general_ci;

-- label
drop table if exists label;
create table if not exists label
(
    internal_id bigint      not null primary key auto_increment,
    id          varchar(32) not null,
    name        varchar(64) not null,
    parent_id   varchar(32) not null,
    extra       json        not null,
    create_time datetime    not null,
    update_time datetime    not null,
    unique key id_uidx (id),
    key parent_id_idx (parent_id),
    unique key name_uidx (name)
) COMMENT 'label' ENGINE = InnoDB
                  DEFAULT CHARSET = utf8mb4;

-- object

drop table if exists object;
create table if not exists object
(
    internal_id    bigint      not null primary key auto_increment,
    id             varchar(32) not null,
    object_type_id varchar(32) not null,
    title          varchar(64) not null default '',
    content        json        not null,
    create_time    datetime    not null,
    update_time    datetime    not null,
    unique key id_uidx (id),
    key object_type_idx (object_type_id)
) COMMENT 'object' ENGINE = InnoDB
                  DEFAULT CHARSET = utf8mb4;

-- object relation
drop table if exists object_rel;
create table if not exists object_rel
(
    internal_id bigint      not null primary key auto_increment,
    parent_id   varchar(32) not null,
    sub_id      varchar(32) not null,
    create_time datetime    not null,
    update_time datetime    not null,
    unique key rel_unique (parent_id, sub_id),
    key parent_id_idx (parent_id),
    key sub_id_idx (sub_id)
) COMMENT 'object' ENGINE = InnoDB
                  DEFAULT CHARSET = utf8mb4;

-- object label relation
drop table if exists object_label_rel;
create table if not exists object_label_rel
(
    internal_id bigint      not null primary key auto_increment,
    object_id    varchar(32) not null,
    label_id    varchar(64) not null,
    create_time datetime    not null,
    unique key object_label_unique (object_id, label_id),
    key label_idx (label_id),
    key object_idx (object_id)
) COMMENT 'object label relation' ENGINE = InnoDB
                                 DEFAULT CHARSET = utf8mb4;

-- collection
drop table if exists collection;
create table if not exists collection
(
    internal_id    bigint      not null primary key auto_increment,
    id             varchar(32) not null,
    object_type_id varchar(32) not null,
    title          varchar(64) not null default '',
    content        json        not null,
    create_time    datetime    not null,
    update_time    datetime    not null,
    unique key id_unique (id),
    key object_type_idx (object_type_id)
) COMMENT 'collection' ENGINE = InnoDB
                       DEFAULT CHARSET = utf8mb4;

-- collection relation
drop table if exists collection_rel;
create table if not exists collection_rel
(
    internal_id bigint      not null primary key auto_increment,
    parent_id   varchar(32) not null,
    sub_id      varchar(32) not null,
    create_time datetime    not null,
    update_time datetime    not null,
    unique key rel_unique (parent_id, sub_id),
    key parent_id_idx (parent_id),
    key sub_id_idx (sub_id)
) COMMENT 'collection' ENGINE = InnoDB
                       DEFAULT CHARSET = utf8mb4;

-- object collection relation
drop table if exists collection_object_rel;
create table if not exists collection_object_rel
(
    internal_id   bigint      not null primary key auto_increment,
    collection_id varchar(32) not null,
    object_id      varchar(32) not null,
    create_time   datetime    not null,
    update_time   datetime    not null,
    unique key collection_object_unique (collection_id, object_id),
    key collection_idx (collection_id),
    key object_idx (object_id)
) COMMENT 'collection object relation' ENGINE = InnoDB
                                      DEFAULT CHARSET = utf8mb4;

-- collection label relation
drop table if exists collection_label_rel;
create table if not exists collection_label_rel
(
    internal_id   bigint      not null primary key auto_increment,
    collection_id varchar(32) not null,
    label_id      varchar(64) not null,
    create_time   datetime    not null,
    unique key collection_label_unique (collection_id, label_id),
    key label_idx (label_id),
    key collection_idx (collection_id)
) COMMENT 'collection label relation' ENGINE = InnoDB
                                      DEFAULT CHARSET = utf8mb4;


-- object_type
drop table if exists object_type;
create table if not exists object_type
(
    internal_id bigint      not null primary key auto_increment,
    id          varchar(32) not null,
    name        varchar(32) not null,
    category    varchar(32) not null,
    description TEXT        not null,
    create_time datetime    not null,
    update_time datetime    not null,
    unique key id_unique (id),
    unique key category_name_unique (category, name)
) COMMENT 'object_type' ENGINE = InnoDB
                        DEFAULT CHARSET = utf8mb4;
