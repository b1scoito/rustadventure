use std::ffi::{c_char, c_float, c_int, c_void, CString};

use color_eyre::Result;
use lazy_static::lazy_static;
use log::{debug, info, warn};
use retour::static_detour;
use widestring::WideCString;
use winapi::um::{
    libloaderapi::{GetModuleHandleW, GetProcAddress},
    winsock2::SOCKET,
};

use crate::memory;

pub mod client_world_tick;
pub mod game_api_tick;
pub mod player_can_jump;
pub mod player_tick;
pub mod ws_send;

// Addresses
lazy_static! {
    static ref FN_PTR: FunctionTypes = FunctionTypes {
        game_api_tick_addr: unsafe {
            std::mem::transmute::<usize, GameAPITickFn>(get_pattern_address(
                "83 EC 28 A1 ? ? ? ? 33 C4 89 44 24 20 53 56 57 A1 ? ? ? ? 33 C4 50 8D 44 24 38 64 A3 ? ? ? ? 89 4C 24 10",
                "GameLogic.dll"
            ) - 0x14)
        },
        client_world_tick_addr: unsafe {
            std::mem::transmute::<usize, ClientWorldTickFn>(get_pattern_address(
                "55 8B EC 51 57 8B F9 8B 4F 2C 85 C9 0F 84 ? ? ? ?",
                "GameLogic.dll",
            ))
        },
        player_can_jump_addr: unsafe {
            std::mem::transmute::<usize, PlayerCanJumpFn>(get_pattern_address(
                "8B 49 9C 85 C9 74 07 8B 01 8B 40 50 FF E0",
                "GameLogic.dll",
            ))
        },
        player_tick_addr: unsafe {
            std::mem::transmute::<usize, PlayerTickFn>(get_pattern_address(
                "55 8B EC 83 E4 C0 6A FF 68",
                "GameLogic.dll",
            ))
        },
        ws_send_addr: unsafe {
            std::mem::transmute::<usize, WsSendFn>(get_function_address("ws2_32.dll", "send"))
        },
    };
}
struct FunctionTypes {
    game_api_tick_addr: GameAPITickFn,
    client_world_tick_addr: ClientWorldTickFn,
    player_can_jump_addr: PlayerCanJumpFn,
    player_tick_addr: PlayerTickFn,
    ws_send_addr: WsSendFn,
}

// Hooks
static_detour! {
    static GameAPITickHook: unsafe extern "fastcall" fn(*mut c_void, *mut c_void, c_float) -> c_void;
    static ClientWorldTickHook: unsafe extern "fastcall" fn(*mut c_void, *mut c_void, c_float) -> c_void;
    static PlayerCanJumpHook: unsafe extern "fastcall" fn(*mut c_void, *mut c_void) -> bool;
    static PlayerTickHook: unsafe extern "fastcall" fn(*mut c_void, *mut c_void, c_float) -> c_void;
    static WsSendHook: unsafe extern "system" fn(SOCKET, *const c_char, c_int, c_int) -> c_int;
}

// Types
type GameAPITickFn = unsafe extern "fastcall" fn(*mut c_void, *mut c_void, c_float) -> c_void;
type ClientWorldTickFn = unsafe extern "fastcall" fn(*mut c_void, *mut c_void, c_float) -> c_void;
type PlayerCanJumpFn = unsafe extern "fastcall" fn(*mut c_void, *mut c_void) -> bool;
type PlayerTickFn = unsafe extern "fastcall" fn(*mut c_void, *mut c_void, c_float) -> c_void;
type WsSendFn = unsafe extern "system" fn(SOCKET, *const c_char, c_int, c_int) -> c_int;

fn get_function_address(module_name: &str, function_name: &str) -> usize {
    let function_name_cstr = CString::new(function_name).unwrap();
    let function_name_cstr_ptr = function_name_cstr.as_ptr();

    let module_name_wcstr = WideCString::from_str(module_name).unwrap();
    let module_name_wcstr_ptr = module_name_wcstr.as_ptr();

    let module_handle = unsafe { GetModuleHandleW(module_name_wcstr_ptr) };
    let address = unsafe { GetProcAddress(module_handle, function_name_cstr_ptr) as usize };

    debug!(
        "Offset for function {} in {}: 0x{:X}",
        function_name, module_name, address
    );

    address
}

fn get_pattern_address(pattern: &str, module_name: &str) -> usize {
    let process = memory::from_pid(memory::get_current_pid()).unwrap();
    let module = process.get_module(module_name).unwrap();
    let address = module.find_pattern(pattern).unwrap();

    debug!(
        "Offset for pattern {} in {}: 0x{:X}",
        pattern,
        module_name,
        address + module.base
    );

    address + module.base
}

pub fn initialize() -> Result<()> {
    info!("Initializing hooks...");

    // Initialize
    unsafe {
        // GameAPI::Tick
        GameAPITickHook
            .initialize(FN_PTR.game_api_tick_addr, game_api_tick::hook)?
            .enable()?;
        info!("Hooked GameAPI::Tick");

        // ClientWorld::Tick
        ClientWorldTickHook
            .initialize(FN_PTR.client_world_tick_addr, client_world_tick::hook)?
            .enable()?;
        info!("Hooked ClientWorld::Tick");

        // Player::CanJump
        PlayerCanJumpHook
            .initialize(FN_PTR.player_can_jump_addr, player_can_jump::hook)?
            .enable()?;
        info!("Hooked Player::CanJump");

        // Player::Tick
        PlayerTickHook
            .initialize(FN_PTR.player_tick_addr, player_tick::hook)?
            .enable()?;
        info!("Hooked Player::Tick");

        // ws2_32!send
        WsSendHook
            .initialize(FN_PTR.ws_send_addr, ws_send::hook)?
            .enable()?;
        info!("Hooked ws2_32!send");
    }

    Ok(())
}

pub fn destroy() -> Result<()> {
    warn!("Destroying hooks...");

    unsafe {
        GameAPITickHook.disable()?;
        ClientWorldTickHook.disable()?;
        PlayerCanJumpHook.disable()?;
        PlayerTickHook.disable()?;
        WsSendHook.disable()?;
    }

    Ok(())
}
