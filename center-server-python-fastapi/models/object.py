#!/usr/bin/env python3.11
# -*- coding: utf-8 -*-

import json

import peewee as pw

import config
import utils

db = pw.SqliteDatabase(config.DB_PATH)


class Object(pw.Model):
    internal_id = pw.AutoField()
    update_time = pw.TextField()
    create_time = pw.TextField()
    id = pw.TextField(unique=True)
    object_type_id = pw.TextField(index=True)
    title = pw.TextField(constraints=[pw.SQL("DEFAULT ''")])
    content = pw.TextField()

    class Meta:
        database: pw.SqliteDatabase = db
        table_name: str = "object"


def get_by_id(id: str) -> Object | None:
    return Object.get_or_none(Object.id == id)
    # return object if object is not None else None


def create(id: str, object_type_id: str, title: str, content: object):
    now = utils.now_beijing()
    Object.insert(
        create_time=now,
        update_time=now,
        id=id,
        object_type_id=object_type_id,
        title=title,
        content=json.dumps(content),
    ).on_conflict_ignore().execute()
    return


def update(
    id: str,
    object_type_id: str | None = None,
    title: str | None = None,
    content: object | None = None,
):
    now = utils.now_beijing()
    updates = {Object.update_time: now}
    if object_type_id is not None:
        updates[Object.object_type_id] = object_type_id
    if title is not None:
        updates[Object.title] = title
    if content is not None:
        updates[Object.content] = json.dumps(content)
    Object.update(updates).where(Object.id == id).execute()
    return
