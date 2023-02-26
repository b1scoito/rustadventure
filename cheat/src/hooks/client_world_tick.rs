use std::ffi::{c_float, c_void};

use crate::hooks::ClientWorldTickHook;

pub fn hook(client_world: *mut c_void, ecx: *mut c_void, delta_time: c_float) -> c_void {
    // TODO: Do I really need this hook?
    unsafe { ClientWorldTickHook.call(client_world, ecx, delta_time) }
}
