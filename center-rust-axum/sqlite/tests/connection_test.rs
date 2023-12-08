use sqlite::Executor;

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
    let raw_rows = conn.fetch_all_raw_rows("select * from t1;", None).unwrap();
    println!("raw_rows {:?}", raw_rows);

    let rows = conn
        .fetch_all_rows::<TestRow>("select * from t1;", None)
        .unwrap();
    println!("rows {:?}", rows);
}

#[test]
fn step() {
    let conn = sqlite::Connection::open_memory("test").unwrap();
    // let conn = sqlite::Connection::open_file("/home/ly/test.db").unwrap();

    conn.execute(
        "create table if not exists t1(id integer not null primary key autoincrement,
        content text not null);",
        None,
    )
    .unwrap();
    conn.execute("create unique index unique_content on t1 (content);", None)
        .unwrap();
    conn.execute(
        "insert into t1(content) values(?);",
        Some(vec!["content1".into()]),
    )
    .unwrap();
    let err = conn.execute(
        "insert into t1(content) values(?);",
        Some(vec!["content1".into()]),
    );
    if err.is_ok() || err.clone().unwrap_err() != sqlite::Error::ConstraintUnique {
        panic!("should be unique constraint error, {:?}", err.unwrap_err());
    }
    let err = conn.execute(
        "insert into t1(id, content) values(?,?);",
        Some(vec![1.into(), "content1".into()]),
    );
    if err.is_ok() || err.clone().unwrap_err() != sqlite::Error::ConstraintPrimaryKey {
        panic!(
            "should be primary key constraint error, {:?}",
            err.unwrap_err()
        );
    }
    let raw_rows = conn.fetch_all_raw_rows("select * from t1;", None).unwrap();
    println!("raw_rows {:?}", raw_rows);

    let rows = conn
        .fetch_all_rows::<TestRow>("select * from t1;", None)
        .unwrap();
    println!("rows {:?}", rows);
    println!("content {}, {}", rows[0].content, rows[0].content.len());
    println!("content {}", "content1".len());
}

#[derive(Default, Debug)]
struct TestRow {
    id: i64,
    content: String,
}

impl sqlite::Row for TestRow {
    fn update_field(&mut self, column_name: &str, value: sqlite::Value) -> &mut Self {
        match column_name {
            "id" => {
                if let sqlite::Value::Integer(v) = value {
                    self.id = v;
                }
            }
            "content" => {
                if let sqlite::Value::Text(v) = value {
                    self.content = v;
                }
            }
            _ => {}
        }
        self
    }
}
