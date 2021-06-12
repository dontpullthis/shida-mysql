use std::mem;

use mysql::prelude::*;

use shida_core::ffi;

use crate::context::reader::ReaderContext;

pub fn read(conn_ptr: *const u8) -> (ffi::ConstCCharPtr, ffi::ConstCCharPtr) {
    let mut reader_context: Box<ReaderContext> = unsafe { mem::transmute(conn_ptr) };
    let test_query: Vec<String> = match reader_context.common_context.mysql_connection.query_map(
        "SELECT \'test123\' LIMIT 1",
        |test| {
            test
        },
    ) {
        Ok(s) => s,
        Err(_) => return (std::ptr::null(), ffi::str_to_ccharptr("Failed to get mysql connection")),
    };
    
    let item = match test_query.get(0) {
        Some(i) => i.clone(),
        None => String::from("WRONG"),
    };


    (ffi::string_to_ccharptr(item), std::ptr::null())
}