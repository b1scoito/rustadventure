use std::ffi::{c_float, c_void};

use crate::hooks::PlayerTickHook;

pub fn hook(player: *mut c_void, ecx: *mut c_void, delta_time: c_float) -> c_void {
    // TODO: Properly read struct
    // let player_read: &mut LocalPlayer = unsafe { &mut *(player as *mut LocalPlayer) };

    // Return original
    unsafe { PlayerTickHook.call(player, ecx, delta_time) }
}
