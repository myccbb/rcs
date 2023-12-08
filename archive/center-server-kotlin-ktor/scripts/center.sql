create database center character set utf8mb4 collate utf8mb4_general_ci;

-- labels
create table if not exists labels
(
    id          bigint      not null primary key auto_increment,
    code        varchar(32) not null,
    name        varchar(64) not null,
    parent_code varchar(32) not null,
    extra       longtext    not null,
    create_time datetime    not null,
    update_time datetime    not null,
    unique key code_uidx (code),
    key parent_code_idx (parent_code),
    unique key name_uidx (name)
) COMMENT 'labels' ENGINE = InnoDB
                   DEFAULT CHARSET = utf8mb4;


-- pieces
create table if not exists pieces
(
    id          bigint      not null primary key auto_increment,
    code        varchar(32) not null,
    type        varchar(32) not null,
    title       varchar(64) not null default '',
    content     longtext    not null,
    create_time datetime    not null,
    update_time datetime    not null,
    unique key code_uidx (code),
    key type_idx (type)
) COMMENT 'pieces' ENGINE = InnoDB
                   DEFAULT CHARSET = utf8mb4;

-- piece label relation
create table if not exists piece_label_rel
(
    id          bigint      not null primary key auto_increment,
    piece_code  varchar(32) not null,
    label_code  varchar(64) not null,
    create_time datetime    not null,
    unique key piece_label_uidx (piece_code, label_code),
    key label_idx (label_code),
    key piece_idx (piece_code)
) COMMENT 'piece label relation' ENGINE = InnoDB
                                 DEFAULT CHARSET = utf8mb4;

-- collections
create table if not exists collections
(
    id          bigint      not null primary key auto_increment,
    code        varchar(32) not null,
    type        varchar(32) not null,
    title       varchar(64) not null default '',
    content     longtext    not null,
    create_time datetime    not null,
    update_time datetime    not null,
    unique key code_uidx (code),
    key type_idx (type)
) COMMENT 'collections' ENGINE = InnoDB
                        DEFAULT CHARSET = utf8mb4;

-- piece collection relation
create table if not exists collection_piece_rel
(
    id              bigint      not null primary key auto_increment,
    collection_code varchar(32) not null,
    piece_code      varchar(32) not null,
    create_time     datetime    not null,
    update_time     datetime    not null,
    unique key collection_piece_uidx (collection_code, piece_code),
    key collection_idx (collection_code),
    key piece_idx (piece_code)
) COMMENT 'collection piece relation' ENGINE = InnoDB
                                      DEFAULT CHARSET = utf8mb4;

-- collection label relation
create table if not exists collection_label_rel
(
    id              bigint      not null primary key auto_increment,
    collection_code varchar(32) not null,
    label_code      varchar(64) not null,
    create_time     datetime    not null,
    unique key collection_label_uidx (collection_code, label_code),
    key label_idx (label_code),
    key collection_idx (collection_code)
) COMMENT 'collection label relation' ENGINE = InnoDB
                                      DEFAULT CHARSET = utf8mb4;


-- types
create table if not exists types
(
    name        varchar(32) not null primary key,
    category    varchar(32) not null,
    create_time datetime    not null,
    update_time datetime    not null,
    unique key name_uidx (name)
) COMMENT 'types' ENGINE = InnoDB
                  DEFAULT CHARSET = utf8mb4;
