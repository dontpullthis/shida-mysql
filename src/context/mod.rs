pub mod reader;

use mysql::{Pool, PooledConn};

pub struct Context {
    pub db_name: Option<String>,
    pub mysql_pool: Pool,
}

impl Context {
    pub fn new(db_name: Option<String>, mysql_pool: Pool) -> Context {
        Context {
            db_name,
            mysql_pool,
        }
    }

    pub fn get_mysql_connection(&self) -> Result<PooledConn, String> {
        match self.mysql_pool.get_conn() {
            Ok(c) => Ok(c),
            Err(e) => Err(format!("Failed to create a connection: {}", e)),
        }
    }
}