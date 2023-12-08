#!/usr/bin/env python3.11
# -*- coding: utf-8 -*-

import json
from datetime import datetime
import sqlite3
import logging

import peewee as pw

import utils
import config
from utils import now_beijing, random_id

db = pw.SqliteDatabase(config.DB_PATH)


class Label(pw.Model):
    id = pw.IntegerField(primary_key=True)
    code = pw.TextField()
    name = pw.TextField()
    parent_code = pw.TextField()
    extra = pw.TextField()
    create_time = pw.DateTimeField()
    update_time = pw.DateTimeField()

    class Meta:
        database: pw.SqliteDatabase = db
        table_name: str = "labels"


def insert(code: str, parent_code: str = "", name: str = "", extra: str = ""):
    now = now_beijing().replace(microsecond=0)
    logging.info("label name %s", name)
    if parent_code is None:
        parent_code = ""
    if extra == "null" or extra is None:
        extra = ""
    r = Label.create(
        code=code,
        parent_code=parent_code,
        name=name,
        extra=extra,
        create_time=now,
        update_time=now,
    )
    return


def get_one(code: str | None = None, name: str | None = None) -> Label | None:
    q = Label.select()
    if code:
        q = q.where(Label.code == code)
    if name:
        q = q.where(Label.name == name)
    return q.get()


def get_many(page=0, page_size=10) -> list[Label] | None:
    q = Label.select()
    q.offset(page * page_size)
    q.limit(page_size)
    return q.execute()


def delete(code: str):
    Label.delete().where(Label.code == code).execute()
    return
