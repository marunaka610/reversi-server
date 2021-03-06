use crate::util::error::CustomError;
use diesel::prelude::*;
use diesel::MysqlConnection;
use dotenv::*;
use std::result::Result;

pub fn establish_connection() -> Result<MysqlConnection, CustomError> {
    dotenv().ok();
    let database_url = "mysql://root:danaLLC0@localhost/reversi";
    match MysqlConnection::establish(&database_url) {
        Ok(connection) => Ok(connection),
        Err(msg) => Err(CustomError::DBConnction(format!(
            "Error connecting to {}: {}",
            database_url, msg
        ))),
    }
}

// pub fn establish_connection_sqlite() -> SqliteConnection {
//   dotenv().ok();

//   let database_url = "reversi.db";
//   SqliteConnection::establish(&database_url)
//     .expect(&format!("Error connecting to {}", database_url))
// }
