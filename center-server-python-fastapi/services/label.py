#!/usr/bin/env python3.11
# -*- coding: utf-8 -*-

from datetime import datetime

class Label:
    id: str
    name: str
    parent_id: str
    extra: dict
    create_time: datetime
    update_time: datetime
