use std::collections::HashMap;

use mysql::Pool;

use shida_core::ffi::app_config::AppConfig;

use crate::context::Context;


pub struct ReaderContext {
    pub app_config: *const AppConfig,
    pub common_context: Context,
    pub cursors: HashMap<String, (usize, usize)>,
}

impl ReaderContext {
    pub fn new(app_config: *const AppConfig, db_name: Option<String>, mysql_pool: Pool) -> ReaderContext {
        ReaderContext {
            app_config,
            common_context: Context::new(db_name, mysql_pool),
            cursors: HashMap::new(),
        }
    }
}