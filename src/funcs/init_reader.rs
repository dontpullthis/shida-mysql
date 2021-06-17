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
    let mut mysql_params = MysqlConnectParams::new();
    let reader_params = match ffi::cchar_ptr_to_vec_string(paramsc, paramsv) {
        Ok(p) => p,
        Err(e) => return (std::ptr::null(), ffi::string_to_ccharptr(format!("Failed to convert param: {}", e))),
    };
    for param in reader_params.iter() {
        let (key, value) = match string_to_keyvalue(&param) {
            Some(r) => r,
            None => continue,
        };

        match key.as_str() {
            "database" => { mysql_params.dbname = Some(value); },
            "host" => { mysql_params.host = Some(value); },
            "password" => { mysql_params.password = Some(value); },
            "port" => { mysql_params.port = Some(value); },
            "user" => { mysql_params.user = Some(value); },
            _ => {},
        };
    }

    let url = format_url(&mysql_params);
    let pool = match Pool::new(url) {
        Ok(p) => p,
        Err(e) => return (std::ptr::null(), ffi::string_to_ccharptr(format!("Failed to create a mysql pool: {}", e))),
    };
    
    match pool.get_conn() {
        Ok(conn) => {
            let context = Box::from(ReaderContext::new(conn));
            (unsafe { mem::transmute(context) }, std::ptr::null())
        },
        Err(e) => ( std::ptr::null(), ffi::string_to_ccharptr(format!("Failed to get mysql connection: {}", e)) ),
    }
}