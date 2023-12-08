pub fn init() -> sqlite::Connection {
    tracing_subscriber::fmt::init();

    let conn = sqlite::Connection::open_memory().unwrap();

    center::db::init_tables_sync(&conn).unwrap();
    conn
}
