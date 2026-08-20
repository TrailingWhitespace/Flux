use turso::{Builder, Connection};

pub async fn init_database() -> Result<Connection, Box<dyn std::error::Error>> {
    let db = Builder::new_local(":memory:").build().await?;
    let conn = db.connect()?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos (id INTEGER PRIMARY KEY AUTOINCREMENT, todo text, completed boolean, completedAt INTEGER);",
        ()
    ).await?;

    // test insert
    conn.execute(
        "INSERT INTO todos (todo, completed) VALUES (?1, ?2);",
        ("Do stuff", false),
    )
    .await?;

    Ok(conn)
}
