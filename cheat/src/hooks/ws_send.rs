use byteorder::{ByteOrder, LittleEndian};
use color_eyre::Result;
use log::{debug, error};
use panic_message::get_panic_message;
use std::{
    ffi::{c_char, c_int},
    ops::Rem,
};
use winapi::um::winsock2::SOCKET;

use crate::{
    adventure::structs::{Vec3, WsBasePacket, WsPlayerInfo},
    hooks::WsSendHook,
};

fn get_ws_base_packet(buffer: &[u8]) -> Result<WsBasePacket> {
    let (identifier_b1, identifier_b2) = buffer.split_at(2);
    let packet_identifier_bytes = LittleEndian::read_i16(&vec![identifier_b1].concat()); // Packet identifier

    Ok(WsBasePacket {
        packet_identifier: packet_identifier_bytes,
        buffer: identifier_b2.to_vec(),
    })
}

fn buf_parse_player(buffer: &[u8]) -> Result<WsPlayerInfo> {
    // Split buffer into bytes
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
    let (forwardfrac_byte1, _forwardfrac_byte2) = rotationz_byte2.split_at(2);

    // Packet position
    let packet_positionx = LittleEndian::read_f32(&vec![positionx_byte1, positionx_byte3].concat());
    let packet_positiony = LittleEndian::read_f32(&vec![positiony_byte1, positiony_byte3].concat());
    let packet_positionz = LittleEndian::read_f32(&vec![positionz_byte1, positionz_byte3].concat());

    // Packet rotation
    let packet_rotationx = LittleEndian::read_i16(&vec![rotationx_byte1].concat()) as f32;
    let packet_rotationy = LittleEndian::read_i16(&vec![rotationy_byte1].concat()) as f32;
    let packet_rotationz = LittleEndian::read_i16(&vec![rotationz_byte1].concat()) as f32;

    // Packet forward fraction
    let forward_fraction_bytes = vec![forwardfrac_byte1];

    // Return parsed packet
    Ok(WsPlayerInfo {
        position: Vec3 {
            x: packet_positionx,
            y: packet_positiony,
            z: packet_positionz,
        },
        rotation: Vec3 {
            x: packet_rotationx,
            y: packet_rotationy,
            z: packet_rotationz,
        },
        forward_fraction: forward_fraction_bytes.concat(),
    })
}

pub fn hook(s: SOCKET, buf: *const c_char, len: c_int, flags: c_int) -> c_int {
    let res = std::panic::catch_unwind(|| {
        // Convert buffer to bytes
        let buffer = unsafe { std::slice::from_raw_parts(buf as *const u8, len as usize) };
        // let base_packet = get_ws_base_packet(buffer).unwrap();
        // debug!(
        //     "Packet identifier: 0x{:X} Buffer: {:?}",
        //     base_packet.packet_identifier, base_packet.buffer
        // );

        if buffer.len() == 22 {
            let player = buf_parse_player(buffer).unwrap();
            let get_pitch = || -> i16 {
                let pitch_normalized: f32 = player.rotation.x.rem(180.0);
                let limit: i32 = (32767.0 * 0.5) as i32 + 1;
                let pitch: i16 = ((limit as f32 / 90.0) * pitch_normalized) as i16;

                pitch
            };

            debug!("Player: {:?} Pitch {}", player, get_pitch());
        }
    });

    if let Err(err) = res {
        error!("Panic called: {}", get_panic_message(&err).unwrap());
    }

    // Return original
    unsafe { WsSendHook.call(s, buf, len, flags) }
}
