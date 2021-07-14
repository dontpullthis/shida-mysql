use std::mem;

use mysql::{OptsBuilder, Pool};
use mysql::prelude::*;

use shida_core::ffi::app_config::AppConfig;
use shida_core::ffi::casting;
use shida_core::ffi::typedefs;
use shida_core::sys::args::string_to_keyvalue;

use crate::context::reader::ReaderContext;

pub fn init_reader(app_config: *const AppConfig, paramsc: typedefs::Size, paramsv: *const typedefs::ConstCCharPtr) -> (*const u8, typedefs::ConstCCharPtr) {
    let reader_params = match casting::cchar_ptr_to_vec_string(paramsc, paramsv) {
        Ok(p) => p,
        Err(e) => return (std::ptr::null(), casting::string_to_ccharptr(format!("Failed to convert param: {}", e))),
    };
    let mut opts_builder = OptsBuilder::new();
    let mut db_name: Option<String> = None;
    for param in reader_params.iter() {
        let (key, value) = match string_to_keyvalue(&param) {
            Some(r) => r,
            None => continue,
        };

        opts_builder = match key.as_str() {
            "database" => { db_name = Some(value.clone()); opts_builder.db_name(Some(value)) },
            "host" => { opts_builder.ip_or_hostname(Some(value)) },
            "password" => { opts_builder.pass(Some(value)) },
            "port" => { opts_builder.tcp_port(value.parse::<u16>().unwrap()) },
            "user" => { opts_builder.user(Some(value)) },
            _ => { opts_builder },
        };
    }

    let pool = match Pool::new(opts_builder) {
        Ok(p) => p,
        Err(e) => return (std::ptr::null(), casting::string_to_ccharptr(format!("Failed to create a mysql pool: {}", e))),
    };
    
    let mut context = Box::from(ReaderContext::new(app_config, db_name, pool));

    match inspect_db(&mut context) {
        Ok(_) => (unsafe { mem::transmute(context) }, std::ptr::null()),
        Err(e) => (std::ptr::null(), casting::string_to_ccharptr(e)),
    }
}

fn inspect_db(context: &mut ReaderContext) -> Result<(), String> {
    let functions = unsafe { &(*context.app_config).functions };
    let mut conn = context.common_context.get_mysql_connection()?;
    let query_all_tables: Vec<String> = match conn.query_map("SHOW TABLES", |table_name| { table_name }) {
        Ok(s) => Ok(s),
        Err(e) => Err(format!("Failed to execute a SQL query: {}", e)),
    }?;
    let db_name = match &context.common_context.db_name {
        Some(d) => Ok(d),
        None => Err(format!("The database name is not provided.")),
    }?;

    for table_name in query_all_tables {
        (functions.log.debug)(casting::string_to_ccharptr(format!("-Discovered a table: {}", &table_name)));
        context.cursors.insert(table_name.clone(), (0, 0));

        let stmt = match conn.prep("SELECT COLUMN_NAME, DATA_TYPE, CHARACTER_MAXIMUM_LENGTH
                FROM INFORMATION_SCHEMA.COLUMNS
                WHERE TABLE_NAME = ?
                    AND TABLE_SCHEMA = ?") {
            Ok(s) => Ok(s),
            Err(e) => Err(format!("Failed to prepare a statement: {}", e)),
        }?;
        let result = match conn.exec_opt::<(String, String, Option<u32>), _, _>(&stmt, (&table_name, &db_name,)) {
            Ok(t) => Ok(t),
            Err(e) => Err(format!("Failed to execute a SQL query: {}", e)),
        }?;

        for r in result {
            let a = r.unwrap();
            (functions.log.debug)(casting::string_to_ccharptr(format!("--Discovered a column: {}: {}{}", a.0, a.1, match a.2 { Some(n) => format!("({})", n), None => String::new(), })));
        }
    }

    Ok(())
}