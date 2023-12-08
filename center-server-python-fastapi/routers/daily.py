#!/usr/bin/env python3.11
# -*- coding: utf-8 -*-

from fastapi import APIRouter
import pydantic as pd

from services import daily as daily_svc
from utils import response_success, response_error

from models import object


router = APIRouter(
    prefix="/center-server/api/v1/daily",
    tags=["daily"],
)


@router.get("")
async def loading():
    daily_record = object.get_by_id(daily_svc.DAILY_ID)
    return response_success(daily_svc.from_db_record(daily_record))


@router.post("")
async def create_collection():
    pass
