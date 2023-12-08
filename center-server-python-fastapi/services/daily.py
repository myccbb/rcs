from datetime import datetime
import json

import pydantic as pd
import strawberry as sb


from models import object_type, object
from services import matter_collection, matter_item

DAILY_ID = "daily"

DAILY_OBJECT_TYPE = "daily"
MATTER_ITEM_OBJECT_TYPE = "matter_item"
MATTER_COLLECTION_OBJECT_TYPE = "matter_collection"


def init_daily():
    object_type.create(DAILY_ID, "daily")
    object_type.create(MATTER_ITEM_OBJECT_TYPE, "matter item")
    object_type.create(MATTER_COLLECTION_OBJECT_TYPE, "matter collection")

    object.create(DAILY_ID, DAILY_OBJECT_TYPE, "daily", "{}")
    return


class DailyContent(pd.BaseModel):
    matter_collections: matter_collection.MatterCollectionRefList


class Daily(pd.BaseModel):
    content: DailyContent | None


def from_db_record(record: object.Object | None) -> Daily | None:
    if record is None:
        return None
    if record.content == "":
        return Daily(matter_collections=None)
    return Daily(**json.loads(record.content))
