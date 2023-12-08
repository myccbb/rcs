from datetime import datetime

import pydantic as pd

from services import matter_item


class MatterCollectionContent(pd.BaseModel):
    matter_items: list[matter_item.MatterItem]
    sub_collections: "list[MatterCollection] | None"


class MatterCollection(pd.BaseModel):
    id: str
    create_time: datetime
    update_time: datetime
    title: str
    content: MatterCollectionContent


# MatterCollection.update_forward_refs()


class MatterCollectionList(pd.RootModel):
    root: list[MatterCollection]


class MatterCollectionRef(pd.BaseModel):
    id: str
    sub_list: "MatterCollectionRefList | None"


class MatterCollectionRefList(pd.RootModel):
    root: list[MatterCollectionRef]
