#!/usr/bin/env python3.11
# -*- coding: utf-8 -*-

from datetime import datetime

import peewee as pw

import config
import utils

db = pw.SqliteDatabase(config.DB_PATH)


class ObjectType(pw.Model):
    internal_id = pw.AutoField()
    create_time = pw.TextField()
    update_time = pw.TextField()
    id = pw.TextField(unique=True)
    name = pw.TextField(unique=True)
    description = pw.TextField()

    class Meta:
        database: pw.SqliteDatabase = db
        table_name: str = "object_type"


def create(id: str, name: str, description: str | None = None):
    now = utils.now_beijing()
    r = (
        ObjectType.insert(
            create_time=now,
            update_time=now,
            id=id,
            name=name,
            description=description if description is not None else "",
        )
        .on_conflict_ignore()
        .execute()
    )
    return
