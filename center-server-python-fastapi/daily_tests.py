import services.daily as d
import services.matter_collection as mc
import utils


def test_daily():
    now = utils.now_beijing()
    daily = d.Daily(content=d.DailyContent(matter_collections=[]))
    daily.content.matter_collections.root.append(
        mc.MatterCollection(
            id="1",
            title="test",
            create_time=now,
            update_time=now,
            content=mc.MatterCollectionContent(
                matter_items=[],
                sub_collections=None,
            ),
        )
    )
    a = daily.model_dump_json()
    assert a == """{"content":{"matter_collections":[{"id":"1"}]}}"""


if __name__ == "__main__":
    test_daily()
