use log::{debug, error};
use panic_message::get_panic_message;
use std::ffi::{c_char, c_int};
use winapi::um::winsock2::SOCKET;

use crate::{
    adventure::structs::{Vec3, WebSocketPlayerInfo},
    hooks::WsSendHook,
};

pub fn hook(s: SOCKET, buf: *const c_char, len: c_int, flags: c_int) -> c_int {
    let res = std::panic::catch_unwind(|| {
        // Convert buffer to bytes
        let buffer = unsafe { std::slice::from_raw_parts(buf as *const u8, len as usize) };

        let get_buffer = || -> f32 {
            // Check if buffer is longer than 2 bytes so we don't panic
            if buffer.len() != 22 {
                return 0.0;
            }

            let (identifier_byte1, identifier_byte2) = buffer.split_at(2);
            let (positionx_byte1, positionx_byte2) = identifier_byte2.split_at(2);
            let (positionx_byte3, positionx_byte4) = positionx_byte2.split_at(2);
            let (positiony_byte1, positiony_byte2) = positionx_byte4.split_at(2);
            let (positiony_byte3, positiony_byte4) = positiony_byte2.split_at(2);
            let (positionz_byte1, positionz_byte2) = positiony_byte4.split_at(2);
            let (positionz_byte3, positionz_byte4) = positionz_byte2.split_at(2);
            let (rotationx_byte1, rotationx_byte2) = positionz_byte4.split_at(2);
            let (rotationy_byte1, rotationy_byte2) = rotationx_byte2.split_at(2);
            let (rotationz_byte1, rotationz_byte2) = rotationy_byte2.split_at(2);
            let (forwardfrac_byte1, forwardfrac_byte2) = rotationz_byte2.split_at(2);

            let player_info = WebSocketPlayerInfo {
                identifier_bytes: Vec::new(),
                position: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                rotation: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                forwardfrac: Vec::new(),
            };

            debug!("Buffer: {:?}", player_info.rotation.x);

            0.0
        };

        get_buffer();

        // debug!(
        //     "Buffer: {:?} pitch: {:?} len: {} flags {}",
        //     buffer,
        //     get_pitch(),
        //     len,
        //     flags
        // );
    });

    if let Err(err) = res {
        error!("Panic called: {}", get_panic_message(&err).unwrap());
    }

    // Return original
    unsafe { WsSendHook.call(s, buf, len, flags) }
}