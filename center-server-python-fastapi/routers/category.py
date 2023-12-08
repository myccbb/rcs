#!/usr/bin/env python3.11
# -*- coding: utf-8 -*-

import fastapi as fapi

router = fapi.APIRouter(
    prefix="/center-server/api/v1/category",
    tags=["category"],
)

