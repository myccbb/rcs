from datetime import datetime


import pydantic as pd


class MatterItemContent(pd.BaseModel):
    sub_items: "list[MatterItem] | None"


class MatterItem(pd.BaseModel):
    id: str
    create_time: datetime
    update_time: datetime
    title: str
    content: MatterItemContent
