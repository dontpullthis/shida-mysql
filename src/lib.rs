mod context;
mod funcs;

use shida_core::module::Module;

use crate::funcs::can_handle::can_handle;
use crate::funcs::init_reader::init_reader;
use crate::funcs::read::read;


#[no_mangle]
fn load() -> Module {
    Module {
        can_handle,
        init_reader,
        read,
    }
}