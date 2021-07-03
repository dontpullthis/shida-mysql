use std::collections::HashMap;

use mysql::PooledConn;

use shida_core::ffi::app_config::AppConfig;

use crate::context::Context;


pub struct ReaderContext {
    pub app_config: *const AppConfig,
    pub common_context: Context,
    pub cursors: HashMap<String, (usize, usize)>,
}

impl ReaderContext {
    pub fn new(app_config: *const AppConfig, mysql_connection: PooledConn) -> ReaderContext {
        ReaderContext {
            app_config,
            common_context: Context::new(mysql_connection),
            cursors: HashMap::new(),
        }
    }
}