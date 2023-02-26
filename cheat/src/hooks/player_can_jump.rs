use std::ffi::c_void;

// use crate::hooks::PlayerCanJumpHook;

pub fn hook(_player: *mut c_void, _ecx: *mut c_void) -> bool {
    // Return original
    // unsafe { PlayerCanJumpHook.call(player, ecx) }
    return true; // Fly
}
