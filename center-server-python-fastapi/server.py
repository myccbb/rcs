#!/usr/bin/env python3.11
# -*- coding: utf-8 -*-

from datetime import datetime, timezone

from fastapi import FastAPI
from pydantic import BaseModel

from routers import label
from routers import daily

from utils import setup_logger

from init import init_db
from services.daily import init_daily

setup_logger()

init_db()
init_daily()

app = FastAPI()

app.include_router(label.router)
app.include_router(daily.router)


@app.get("/")
async def root():
    return {"message": "Hello World"}
