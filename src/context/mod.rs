pub mod reader;

use mysql::PooledConn;

pub struct Context {
    pub mysql_connection: PooledConn
}

impl Context {
    pub fn new(mysql_connection: PooledConn) -> Context {
        Context {
            mysql_connection,
        }
    }
}