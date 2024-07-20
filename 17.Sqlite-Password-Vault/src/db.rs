/*
 * @Author: yk
 * @Date: 2024-07-20 14:12:33
 * @Description: 
 */

use std::io;
use std::io::Write;

use rusqlite::Error;
use rusqlite::Connection;
use serde::Serialize;
use serde::Deserialize;

pub fn init_database() -> Result<Connection,Error>{
    let conn = Connection::open("MyVault.db")?;

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS passwords(
            id              INTEGER PRIMARY KEY,
            server          TEXT,
            user            TEXT,
            password        Text
        )",
        [],
    )?;

        Ok(conn)
}

pub fn write_pwd_to_db(
    conn: &Connection,
    server: &str,
    user: &str,
    password: &str,
)-> Result<(),Error>{
    conn.execute(
    "INSERT INTO passwords (server, user, password) VALUES (?, ?, ?)",    
        &[&server, &user, &password], 
    )?;
    Ok(())
}

pub fn read_pwd_from_db(conn: &Connection)-> Result<Vec<ServiceInfo>,Error>{
    let mut info = conn.prepare("
        SELECT server, user, password from passwords
    ")?;

    let entries = info
        .query_map([], |row| {
            Ok(ServiceInfo::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                ))
        })?
        .collect::<Result<Vec<_>,_>>()?;

    Ok(entries)
}

pub fn search_pwd_from_db(conn: &Connection, servername: &str) -> Result<Option<ServiceInfo>,Error>{
    let mut info = conn.prepare("SELECT id,server,user,password from passwords where server = ?")?;

    let result = info.query_row([&servername], |row|{
        Ok(ServiceInfo{
            id: Some(row.get(0)?),
            server: row.get(1)?,
            user: row.get(2)?,
            password: row.get(3)?,
        })
    });

    match result{
        Ok(entry) => Ok(Some(entry)),
        Err(Error::QueryReturnedNoRows) => Ok(None),
        Err(err) => Err(err),
    }
}

 #[derive(Serialize,Deserialize)]
 pub struct ServiceInfo{
    pub id: Option<i64>,
    pub server: String,
    pub user: String,
    pub password: String
 }

 impl ServiceInfo {

    pub fn new(server: String, user: String, password: String) -> Self{
        ServiceInfo{
            id: None,
            server,
            user,
            password
        }     
    }
     
 }

 pub fn promt(promt: &str) -> String{
    print!("{}",promt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
    
 }