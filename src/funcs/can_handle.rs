use shida_core::ffi;
use shida_core::ffi::typedefs;

pub fn can_handle(connection_type: typedefs::ConstCCharPtr) -> bool {
    let connection_type_str = match ffi::ccharptr_to_string(connection_type) {
        Ok(s) => s,
        Err(_) => return false,
    };
    connection_type_str == "mysql"
}