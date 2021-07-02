use std::collections::HashMap;

use mysql::PooledConn;

use crate::context::Context;


pub struct ReaderContext {
    pub common_context: Context,
    pub cursors: HashMap<String, (usize, usize)>,
}

impl ReaderContext {
    pub fn new(mysql_connection: PooledConn) -> ReaderContext {
        ReaderContext {
            common_context: Context::new(mysql_connection),
            cursors: HashMap::new(),
        }
    }
}