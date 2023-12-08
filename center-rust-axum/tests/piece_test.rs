mod common;

#[tokio::test]
async fn test_piece_operation() {
    let conn = common::init();
    let _tx = conn.begin().await.unwrap();

    let now = center::utils::now_beijing();
    let mut obj = center::db::Object {
        internal_id: 0,
        create_time: now,
        update_time: now,
        id: "test_id".to_string(),
        piece_type_id: "test_piece_type_id".to_string(),
        content: r#"{"hello_key":"hello_value"}"#.to_string(),
    };

    obj.create(&conn).await.unwrap();

    log::info!("obj: {:?}", obj);

    let result = obj.create(&conn).await;
    if let Err(e) = result {
        log::info!("anyhow error: {:?}", e);
        if let Some(e) = e.downcast_ref::<sqlx::Error>() {
            log::info!("sqlx error: {:?}", e);
            assert_eq!(
                center::db::Error::from(e),
                center::db::Error::DuplicateEntry
            )
        }
    }

    let new_obj = center::db::Object::get_by_id(&conn, "test_id")
        .await
        .unwrap();

    log::info!("obj: {:?}", obj);
    log::info!("new_obj: {:?}", new_obj);

    assert_eq!(obj, new_obj);

    obj.content = r#"{"hello_key":"hello_value2"}"#.to_string();
    obj.update_by_id(&conn).await.unwrap();
    let new_obj = center::db::Object::get_by_id(&conn, "test_id")
        .await
        .unwrap();

    log::info!("obj: {:?}", obj);
    log::info!("new_obj: {:?}", new_obj);
    assert_eq!(obj, new_obj);

    let result = center::db::Object::get_by_id(&conn, "999").await;
    if let Err(e) = result {
        assert_eq!(e, center::db::Error::RecordNotFound)
    }
}
