use std::mem;

use mysql::prelude::*;

use shida_core::ffi::casting;
use shida_core::ffi::typedefs;

use crate::context::reader::ReaderContext;

pub fn read(conn_ptr: *const u8) -> (typedefs::ConstCCharPtr,typedefs::ConstCCharPtr) {
    let reader_context: Box<ReaderContext> = unsafe { mem::transmute(conn_ptr) };
    let mut conn = match reader_context.common_context.get_mysql_connection() {
        Ok(c) => c,
        Err(e) => return (std::ptr::null(), casting::string_to_ccharptr(e)),
    };

    let test_query: Vec<String> = match conn.query_map(
        "SHOW TABLES",
        |test| {
            test
        },
    ) {
        Ok(s) => s,
        Err(e) => return (std::ptr::null(), casting::string_to_ccharptr(format!("Failed to execute a SQL query: {}", e))),
    };
    
    let item = match test_query.get(0) {
        Some(i) => i.clone(),
        None => String::from("WRONG"),
    };


    (casting::string_to_ccharptr(item), std::ptr::null())
}