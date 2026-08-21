use turso::{Builder, Connection};

pub async fn init_database() -> Result<Connection, Box<dyn std::error::Error>> {
    let db_path = std::env::var("DATABASE_PATH").unwrap_or_else(|_| "./data/flux.db".to_string());
    // ./ is the dir where cargo run is run from i guess,
    // maybe change this to always be at project root like backend/data/flux.db in env and here
    let db = Builder::new_local(&db_path).build().await?;
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
