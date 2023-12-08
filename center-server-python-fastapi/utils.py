#!/usr/bin/env python3.11
# -*- coding: utf-8 -*-

from datetime import datetime, timezone, timedelta
import logging
import random, string

import status_code as code

LABEL_ID_PREFIX = "label"
PIECE_ID_PREFIX = "piece"


def random_id(prefix):
    now = datetime.utcnow()
    rand_id = "".join(
        random.choice(string.ascii_uppercase + string.digits) for _ in range(6)
    )
    return prefix + now.strftime("-%y%m%d-%H%M%S-") + rand_id


BJTZ = timezone(timedelta(hours=8))


def now_beijing():
    return (
        datetime.utcnow()
        .replace(microsecond=0)
        .replace(tzinfo=timezone.utc)
        .astimezone(BJTZ)
    )


def setup_logger():
    logging.basicConfig(
        level=logging.DEBUG,
        format=(
            """%(asctime)s %(levelname)s %(processName)s:%(process)d """
            """%(name)s %(pathname)s:%(lineno)d %(funcName)s(): %(message)s"""
        ),
    )
    return


def response_success(data=None):
    res = {
        "code": code.Success,
        "msg": "success",
    }
    if data is not None:
        res["data"] = data
    return res


def response_error(res_code, msg):
    return {
        "code": res_code,
        "msg": msg,
    }
