create database if not exists center character set utf8mb4 collate utf8mb4_general_ci;

-- label
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

-- piece
create table if not exists piece
(
    internal_id bigint      not null primary key auto_increment,
    id          varchar(32) not null,
    type_id     varchar(32) not null,
    title       varchar(64) not null default '',
    content     json        not null,
    create_time datetime    not null,
    update_time datetime    not null,
    unique key id_uidx (id),
    key type_idx (type_id)
) COMMENT 'piece' ENGINE = InnoDB
                   DEFAULT CHARSET = utf8mb4;

-- piece relation
create table if not exists piece_rel
(
    internal_id     bigint      not null primary key auto_increment,
    parent_id       varchar(32) not null,
    sub_id          varchar(32) not null,
    create_time     datetime    not null,
    update_time     datetime    not null,
    unique key rel_uidx (parent_id, sub_id),
    key parent_id_idx (parent_id),
    key sub_id_idx (sub_id)
) COMMENT 'piece' ENGINE = InnoDB
                   DEFAULT CHARSET = utf8mb4;

-- piece label relation
create table if not exists piece_label_rel
(
    internal_id bigint      not null primary key auto_increment,
    piece_id    varchar(32) not null,
    label_id    varchar(64) not null,
    create_time datetime    not null,
    unique key piece_label_uidx (piece_id, label_id),
    key label_idx (label_id),
    key piece_idx (piece_id)
) COMMENT 'piece label relation' ENGINE = InnoDB
                                 DEFAULT CHARSET = utf8mb4;

-- collection
create table if not exists collection
(
    internal_id bigint      not null primary key auto_increment,
    id          varchar(32) not null,
    type_id     varchar(32) not null,
    title       varchar(64) not null default '',
    content     json        not null,
    create_time datetime    not null,
    update_time datetime    not null,
    unique key id_uidx (id),
    key type_idx (type_id)
) COMMENT 'collection' ENGINE = InnoDB
                        DEFAULT CHARSET = utf8mb4;

-- collection relation
create table if not exists collection_rel
(
    internal_id bigint      not null primary key auto_increment,
    parent_id   varchar(32) not null,
    sub_id      varchar(32) not null,
    create_time datetime    not null,
    update_time datetime    not null,
    unique key rel_uidx (parent_id, sub_id),
    key parent_id_idx (parent_id),
    key sub_id_idx (sub_id)
) COMMENT 'collection' ENGINE = InnoDB
                        DEFAULT CHARSET = utf8mb4;

-- piece collection relation
create table if not exists collection_piece_rel
(
    internal_id   bigint      not null primary key auto_increment,
    collection_id varchar(32) not null,
    piece_id      varchar(32) not null,
    create_time   datetime    not null,
    update_time   datetime    not null,
    unique key collection_piece_uidx (collection_id, piece_id),
    key collection_idx (collection_id),
    key piece_idx (piece_id)
) COMMENT 'collection piece relation' ENGINE = InnoDB
                                      DEFAULT CHARSET = utf8mb4;

-- collection label relation
create table if not exists collection_label_rel
(
    internal_id   bigint      not null primary key auto_increment,
    collection_id varchar(32) not null,
    label_id      varchar(64) not null,
    create_time   datetime    not null,
    unique key collection_label_uidx (collection_id, label_id),
    key label_idx (label_id),
    key collection_idx (collection_id)
) COMMENT 'collection label relation' ENGINE = InnoDB
                                      DEFAULT CHARSET = utf8mb4;


-- object_type
create table if not exists object_type
(
    internal_id bigint      not null primary key auto_increment,
    id          varchar(32) not null,
    name        varchar(32) not null,
    category    varchar(32) not null,
    create_time datetime    not null,
    update_time datetime    not null,
    unique key id_uidx (id),
    unique key category_name_uidx (category, name)
) COMMENT 'object_type' ENGINE = InnoDB
                      DEFAULT CHARSET = utf8mb4;
