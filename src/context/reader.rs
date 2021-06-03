use crate::context::Context;

use mysql::PooledConn;

pub struct ReaderContext {
    pub common_context: Context,
}

impl ReaderContext {
    pub fn new(mysql_connection: PooledConn) -> ReaderContext {
        ReaderContext {
            common_context: Context::new(mysql_connection),
        }
    }
}