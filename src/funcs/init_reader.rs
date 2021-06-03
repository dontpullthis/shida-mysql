use std::mem;

use mysql::Pool;

use shida_core::ffi;

use crate::context::reader::ReaderContext;

pub fn init_reader(_paramsc: ffi::Size, _paramsv: *const ffi::ConstCCharPtr) -> (*const u8, ffi::ConstCCharPtr) {
    let url = "mysql://myuser:1234@localhost:3306/sample_db";

    let pool = match Pool::new(url) {
        Ok(p) => p,
        Err(_) => return unsafe { 
            (std::ptr::null(), ffi::string_to_ccharptr(String::from("Failed to create a mysql pool")))
        },
    };
    
    match pool.get_conn() {
        Ok(conn) => {
            let connection = Box::from(ReaderContext::new(conn));
            (unsafe { mem::transmute(connection) }, std::ptr::null())
        },
        Err(_) => unsafe { ( std::ptr::null(), ffi::string_to_ccharptr(String::from("Failed to get mysql connection")) ) },
    }
}