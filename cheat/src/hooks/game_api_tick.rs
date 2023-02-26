use std::ffi::{c_float, c_void};

use crate::hooks::GameAPITickHook;

pub fn hook(game_api: *mut c_void, ecx: *mut c_void, delta_time: c_float) -> c_void {
    // TODO: Do I really need this hook?
    unsafe { GameAPITickHook.call(game_api, ecx, delta_time) }
}
