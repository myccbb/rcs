use sqlite::Executor;
use sqlite::Row;

#[test]
fn simple_struct() {
    let mut s = SimpleStruct::default();
    s.update_field("hello", sqlite::Value::Text("hello_value".into()));
    s.update_field("content", "hello content".into());
    s.update_field("id", 999.into());
    println!("s: {:?}", s);
}

#[derive(sqlite_macros::Row_old, Debug, Default)]
struct SimpleStruct {
    id: i64,
    content: String,
}

#[test]
fn execute() {
    let conn = sqlite::Connection::open_memory("test").unwrap();
    // let conn = sqlite::Connection::open_file("/home/ly/test.db").unwrap();

    conn.execute(
        "create table if not exists t1(id integer not null primary key autoincrement,
        content text not null);",
        None,
    )
    .unwrap();
    conn.execute("delete from t1;", None).unwrap();
    conn.execute(
        "insert into t1(content) values(?);",
        Some(vec!["中文测试".into()]),
    )
    .unwrap();
    conn.execute(
        "insert into t1(content) values(?);",
        Some(vec!["中文测试1".into()]),
    )
    .unwrap();
    let raw_rows = conn.fetch_all_raw_rows("select * from t1;", None).unwrap();
    println!("raw_rows {:?}", raw_rows);

    let rows = conn
        .fetch_all_rows::<TestRow>("select * from t1;", None)
        .unwrap();
    println!("rows {:?}", rows);
}

#[derive(Default, Debug, sqlite_macros::Row_old)]
struct TestRow {
    id: i64,
    content: String,
}
