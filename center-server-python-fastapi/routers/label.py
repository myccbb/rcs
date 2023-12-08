#!/usr/bin/env python3.11
# -*- coding: utf-8 -*-


import json
from datetime import datetime
import logging

from fastapi import APIRouter
from pydantic import BaseModel

from config import DB_PATH
from models import labels
import utils
from utils import random_id, response_success, response_error
import status_code

router = APIRouter(
    prefix="/center-server/api/v1/labels",
    tags=["labels"],
)


class LabelItem(BaseModel):
    code: str
    name: str
    parent_code: str
    extra: dict | None
    create_time: datetime
    update_time: datetime


@router.get("/list")
async def list_labels(page = 0, page_size = 10):
    label_list = labels.get_many(page=page, page_size=page_size)
    results = []
    for i in label_list:
        label = LabelItem(
            code=i.code,
            name=i.name,
            parent_code=i.code,
            extra=None,
            create_time=datetime.fromisoformat(i.create_time),
            update_time=datetime.fromisoformat(i.update_time),
        )
        if i.extra:
            label.extra = json.loads(i.extra)
        results.append(label)
    return res_success(results)


@router.get("")
async def get_label(label_id: str | None, label_name: str | None):
    r = labels.get_one(code=label_id, name=label_name)
    label = LabelItem(
        code=r.code,
        name=r.name,
        parent_code=r.code,
        extra=None,
        create_time=datetime.fromisoformat(r.create_time),
        update_time=datetime.fromisoformat(r.update_time),
    )
    return label


class CreateLabelReq(BaseModel):
    name: str
    parent_id: str | None
    extra: dict | None


@router.post("")
async def create_label(req: CreateLabelReq):
    logging.info("into create_label %s", req)
    try:
        labels.insert(code=random_id(utils.LABEL_ID_PREFIX),
                      name=req.name, parent_code=req.parent_id, extra=json.dumps(req.extra))
    except Exception as e:
        logging.info(e)
        return response_error(status_code.DBError, str(e))
    return response_success()


@router.put("")
async def edit_label(
        label_id: str,
        name: str | None,
        parent_id: str | None,
        extra: dict | None):
    return {}


@router.delete("")
async def delete_label(label_id: str):
    return labels.delete(code=label_id)
