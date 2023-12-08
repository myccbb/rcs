use sqlite::Value;

#[test]
fn value_array() {
    let _: Vec<crate::Value> = vec![1.into(), "a".into()];
    // panic!("err");
    let offset = chrono::FixedOffset::east_opt(8 * 3600).unwrap();
    use chrono::SubsecRound;
    use chrono::TimeZone;
    let datetime = offset.from_utc_datetime(&chrono::Utc::now().naive_utc().trunc_subsecs(0));
    let v = crate::Value::from(datetime);
    let datetime: chrono::DateTime<chrono::FixedOffset> = v.into();

    let v = crate::Value::from(datetime);
    let _s: String = v.into();
}
