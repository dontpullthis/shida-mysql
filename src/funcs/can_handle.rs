use shida_core::ffi;

pub fn can_handle(connection_type: ffi::ConstCCharPtr) -> bool {
    let connection_type_str = unsafe {
        match ffi::ccharptr_to_string(connection_type) {
            Ok(s) => s,
            Err(_) => return false,
        }
    };
    connection_type_str == "mysql"
}