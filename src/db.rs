use sqlx::Connection;
use serde::{Deserialize, Serialize};
use sqlx::Row;

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Entry {
    pub id: i32,
    pub key: String,
    pub value: String,
    pub hash: String,
    pub created_at: String,
}


async fn get_db() -> sqlx::Result<sqlx::SqliteConnection> {
    //used to connect to the DATABASE_URL
    // It can be any valid SQLite connection string
    sqlx::SqliteConnection::connect(format!("sqlite:{}", std::env::var("KEY_STOR").unwrap()).as_str()).await
}

pub async fn connect() -> sqlx::SqliteConnection {
    //create table todo if not exists
    let mut conn = get_db().await.expect("Error connecting to db");
    let _ = sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS entries (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            key TEXT NOT NULL,
            value TEXT NOT NULL
            hash TEXT NOT NULL,
            created_at TEXT NOT NULL
        )
        "#,
    ).execute(&mut conn).await.unwrap();
    conn
}

pub async fn get_all() -> sqlx::Result<Vec<Entry>> {
    let mut conn = get_db().await.expect("Error connecting to db");
    let entries = sqlx::query_as::<_, Entry>("SELECT * FROM entries")
        .fetch_all(&mut conn)
        .await?;
    Ok(entries)
}

pub async fn get_by_id(id: i32) -> sqlx::Result<Entry> {
    let mut conn = get_db().await.expect("Error connecting to db");
    let entry = sqlx::query_as::<_, Entry>("SELECT * FROM entries WHERE id = ?")
        .bind(id)
        .fetch_one(&mut conn)
        .await?;
    Ok(entry)
}

pub async fn get_by_key(key: String) -> sqlx::Result<Entry> {
    let mut conn = get_db().await.expect("Error connecting to db");
    let entry = sqlx::query_as::<_, Entry>("SELECT * FROM entries WHERE key = ?")
        .bind(key)
        .fetch_one(&mut conn)
        .await?;
    Ok(entry)
}

//add to db
pub async fn add(key: String, value: String, hash: String) -> sqlx::Result<Entry> {
    let mut conn = get_db().await.expect("Error connecting to db");
    let entry = sqlx::query_as::<_, Entry>("INSERT INTO entries (key, value, hash, created_at) VALUES (?, ?, ?, ?)")
        .bind(key)
        .bind(value)
        .bind(hash)
        .bind(chrono::Local::now().to_string())
        .fetch_one(&mut conn)
        .await?;
    Ok(entry)
}

//update db
pub async fn update(id: i32, key: String, value: String, hash: String) -> sqlx::Result<Entry> {
    let mut conn = get_db().await.expect("Error connecting to db");
    let entry = sqlx::query_as::<_, Entry>("UPDATE entries SET key = ?, value = ?, hash = ? WHERE id = ?")
        .bind(key)
        .bind(value)
        .bind(hash)
        .bind(id)
        .fetch_one(&mut conn)
        .await?;
    Ok(entry)
}

//delete from db
pub async fn delete(id: i32) -> sqlx::Result<Entry> {
    let mut conn = get_db().await.expect("Error connecting to db");
    let entry = sqlx::query_as::<_, Entry>("DELETE FROM entries WHERE id = ?")
        .bind(id)
        .fetch_one(&mut conn)
        .await?;
    Ok(entry)
}

//get keys
pub async fn get_keys() -> sqlx::Result<Vec<String>> {
    let mut conn = get_db().await.expect("Error connecting to db");
    let keys = sqlx::query("SELECT key FROM entries")
        .fetch_all(&mut conn)
        .await?;
    let mut key_vec: Vec<String> = Vec::new();
    for key in keys {
        key_vec.push(key.get(0));
    }
    Ok(key_vec)
}