use sqlite::Executor;

#[derive(Debug, Default)]
struct T1 {
    id: i64,
    content: String,
}

impl sqlite::Row for T1 {
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

#[test]
fn rollback() {
    let conn = sqlite::Connection::open_memory("test").unwrap();

    conn.execute(
        "create table if not exists t1(id integer not null primary key autoincrement,
        content text not null);",
        None,
    )
    .unwrap();

    if true {
        let tx = conn.begin_transaction().unwrap();
        tx.execute(
            "insert into t1(id, content) values(?,?);",
            Some(vec![1.into(), "hello".into()]),
        )
        .unwrap();
        let _ = tx.commit();
        // drop(tx);
    }
    let row = conn
        .fetch_all_rows::<T1>("select * from t1;", None)
        .unwrap();
    println!("row: {:?}", row);
}
