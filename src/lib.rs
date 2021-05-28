use shida_core::module::Module;
use shida_core::ffi::const_c_char_ptr;
use shida_core::ffi::ccharptr_to_string;

fn can_handle(connection_type: const_c_char_ptr) -> bool {
    let connection_type_str = unsafe {
        match ccharptr_to_string(connection_type) {
            Ok(s) => s,
            Err(_) => return false,
        }
    };
    connection_type_str == "mysql"
}

// Is there any point in providing a structure here?
// Maybe functions can be exported instead?
// Reason to keep it as structure: in client app a function can be used instead of Symbol
#[no_mangle]
fn load() -> Module {
    Module {
        can_handle: can_handle,
    }
}