use log::{error, info, warn};
use std::{ffi::c_void, net::TcpStream, sync::Arc};
use tracing_subscriber::filter::LevelFilter;
use winapi::{
    shared::minwindef::{BOOL, HINSTANCE, TRUE},
    um::{
        libloaderapi::{DisableThreadLibraryCalls, FreeLibraryAndExitThread},
        processthreadsapi::CreateThread,
        utilapiset::Beep,
        winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
    },
};

mod adventure;
mod hooks;
mod keyboard;
mod memory;

static mut DLL_INSTANCE: HINSTANCE = std::ptr::null_mut();

unsafe extern "system" fn dll_attach(_: *mut c_void) -> u32 {
    color_eyre::install().unwrap();

    let stream: Arc<TcpStream> = Arc::new(TcpStream::connect("127.0.0.1:1337").unwrap());
    let stream_clone: Arc<TcpStream> = stream.clone();

    tracing_subscriber::fmt()
        .compact()
        .with_max_level(LevelFilter::TRACE)
        .with_writer(stream_clone)
        .init();

    info!("Module loaded");

    // Load
    match adventure::load() {
        Ok(result) => result,
        Err(err) => {
            error!("adventure on load panicked! {:?}", err); // Print error
        }
    };

    // Unload
    match adventure::unload() {
        Ok(result) => result,
        Err(err) => {
            error!("adventure on unload panicked! {:?}", err); // Print error
        }
    };

    // Close socket connection.
    warn!("Closing socket connection...");
    stream.as_ref().shutdown(std::net::Shutdown::Both).unwrap();

    // Detach DLL when loop routine finishes.
    FreeLibraryAndExitThread(DLL_INSTANCE, 0);

    // Return TRUE
    1
}

#[no_mangle]
pub extern "system" fn DllMain(h_module: HINSTANCE, dw_reason: u32, _: *mut c_void) -> BOOL {
    match dw_reason {
        DLL_PROCESS_ATTACH => unsafe {
            DLL_INSTANCE = h_module;
            DisableThreadLibraryCalls(DLL_INSTANCE);

            CreateThread(
                std::ptr::null_mut(),
                0,
                Some(dll_attach),
                std::ptr::null_mut(),
                0,
                std::ptr::null_mut(),
            );

            Beep(600, 300);
        },
        DLL_PROCESS_DETACH => unsafe {
            Beep(500, 300);
        },
        _ => {}
    }

    TRUE
}
