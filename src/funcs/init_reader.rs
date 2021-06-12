use std::mem;

use mysql::Pool;

use shida_core::ffi;
use shida_core::sys::args::string_to_keyvalue;

use crate::context::reader::ReaderContext;

struct MysqlConnectParams {
    dbname: Option<String>,
    host: Option<String>,
    password: Option<String>,
    port: Option<String>,
    user: Option<String>,
}

impl MysqlConnectParams {
    fn new() -> MysqlConnectParams {
        MysqlConnectParams {
            dbname: None,
            host: None,
            password: None,
            port: None,
            user: None,
        }
    }
}

fn format_separated_string(args: [&Option<String>; 2]) -> String {
    Vec::from(args).iter()
        .filter(|i| i.is_some())
        .map(|i| i.as_ref().unwrap().clone())
        .collect::<Vec<String>>()
        .join(":")
}

fn format_url(params: &MysqlConnectParams) -> String {
    let userpass = format_separated_string([&params.user, &params.password]);
    let hostport = format_separated_string([&params.host, &params.port]);
    let dbname = match &params.dbname {
        Some(d) => d.clone(),
        None => String::new(),
    };

    format!("mysql://{}@{}/{}", userpass, hostport, dbname)
}

pub fn init_reader(paramsc: ffi::Size, paramsv: *const ffi::ConstCCharPtr) -> (*const u8, ffi::ConstCCharPtr) {
    let mut params = MysqlConnectParams::new();
    for i in 0..paramsc {
        let ch: ffi::ConstCCharPtr = unsafe { *paramsv.offset(i as isize) };
        let param = match ffi::ccharptr_to_string(ch) {
            Ok(string) => string,
            Err(_) => return (std::ptr::null(), ffi::str_to_ccharptr("Failed to convert param")),
        };
        let (key, value) = match string_to_keyvalue(&param) {
            Some(r) => r,
            None => continue,
        };

        match key.as_str() {
            "database" => { params.dbname = Some(value); },
            "host" => { params.host = Some(value); },
            "password" => { params.password = Some(value); },
            "port" => { params.port = Some(value); },
            "user" => { params.user = Some(value); },
            _ => {},
        };
    }

    let url = format_url(&params);
    let pool = match Pool::new(url) {
        Ok(p) => p,
        Err(_) => return (std::ptr::null(), ffi::str_to_ccharptr("Failed to create a mysql pool")),
    };
    
    match pool.get_conn() {
        Ok(conn) => {
            let context = Box::from(ReaderContext::new(conn));
            (unsafe { mem::transmute(context) }, std::ptr::null())
        },
        Err(_) => ( std::ptr::null(), ffi::str_to_ccharptr("Failed to get mysql connection") ),
    }
}