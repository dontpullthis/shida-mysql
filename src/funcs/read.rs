use std::mem;

use mysql::prelude::*;

use shida_core::ffi;
use shida_core::ffi::typedefs;

use crate::context::reader::ReaderContext;

pub fn read(conn_ptr: *const u8) -> (typedefs::ConstCCharPtr,typedefs::ConstCCharPtr) {
    let mut reader_context: Box<ReaderContext> = unsafe { mem::transmute(conn_ptr) };
    let test_query: Vec<String> = match reader_context.common_context.mysql_connection.query_map(
        // "SELECT \'test123\' LIMIT 1",
        "SHOW TABLES",
        |test| {
            test
        },
    ) {
        Ok(s) => s,
        Err(e) => return (std::ptr::null(), ffi::string_to_ccharptr(format!("Failed to execute a SQL query: {}", e))),
    };
    
    let item = match test_query.get(0) {
        Some(i) => i.clone(),
        None => String::from("WRONG"),
    };


    (ffi::string_to_ccharptr(item), std::ptr::null())
}