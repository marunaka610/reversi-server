// use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::{
  MysqlConnection,
  //
  SqliteConnection,
};
use dotenv::*;

pub fn establish_connection() -> MysqlConnection {
  dotenv().ok();

  let database_url = "mysql://root:danaLLC0@localhost/reversi";
  // let database_url = "reversi.db";
  MysqlConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn establish_connection_sqlite() -> SqliteConnection {
  dotenv().ok();

  let database_url = "reversi.db";
  SqliteConnection::establish(&database_url)
    .expect(&format!("Error connecting to {}", database_url))
}
