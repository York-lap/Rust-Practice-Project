use std::result::Result;
use sqlx::{migrate::MigrateDatabase, sqlite::SqliteQueryResult, Sqlite, SqlitePool};

async fn create_schema(db_url: &str) -> Result<SqliteQueryResult,sqlx::Error> {
    
    // 1.We need to connect to Sqlite
    let pool = SqlitePool::connect(&db_url).await?;

    // 2.Create tables
    let qry = 
    "
      CREATE TABLE IF NOT EXISTS settings
      (
        setting_id              INTEGER PRIMARY KEY NOT NULL,
        description             TEXT                NOT NULL,
        created_on              DATETIME DEFAULT (datetime('now','localtime')),
        done                    BOOLEAN             NOT NULL DEFAULT 0
      );
    ";

    let result = sqlx::query(&qry).execute(&pool).await;

    // 3.Close connect and return result
    pool.close().await;
    return result;



}

// Similar with tokio
#[async_std::main]
async fn main() {

    // 1.We need Create Database if not exist
    let db_url = String::from("sqlite://sqlite.db"); // Set our db_url
    
    // `Sqlite::database_exists(&db_url)` will return a `future`
    // we need use await to recept it.If something wrong like `not exists` 
    // we use `false` to mark. But we add `!`, result have reserverd.
    // database not exists == true,so we excute code in `if{}``

    if !Sqlite::database_exists(&db_url).await.unwrap_or(false){

        // (1) create database
        Sqlite::create_database(&db_url).await.unwrap();
        
        // (2) create tables & deal with result
        match create_schema(&db_url).await {
            Ok(_) => println!("Database Created Successfully!"),
            Err(e) => println!("{}",e),
        
        }
    }
    // 2.Connect Test & Insert data Test
    let instance = SqlitePool::connect(&db_url).await.unwrap();
    // we insert value `$1` into table:settings -- filed:description 
    let qry = "INSERT INTO settings (description) values($1)";
    let result = sqlx::query(&qry).bind("testing").execute(&instance).await;

    // 3.Close the connect
    instance.close().await;

    // 4.Put the result
    println!("{:?}",result);

}
