use std::{
    env,
    io::{Read, Write},
    net::TcpListener,
    path::PathBuf,
};

use color_eyre::Report;
use dll_syringe::{process::OwnedProcess, Syringe};
use log::{error, info, warn};
use tracing_subscriber::filter::LevelFilter;

fn get_current_dir() -> Result<PathBuf, Report> {
    let mut current_exe = env::current_exe()?;
    current_exe.pop();

    Ok(current_exe)
}

const PROCESS_NAME: &str = "PwnAdventure3-Win32-Shipping.exe";

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt()
        .compact()
        .with_max_level(LevelFilter::TRACE)
        .init();

    let listener = TcpListener::bind("127.0.0.1:1337")?;

    let mut dll_path = get_current_dir()?;
    dll_path.push("rustadventure.dll");

    info!("Injecting {} into {}.", dll_path.display(), PROCESS_NAME);

    let process = OwnedProcess::find_first_by_name(PROCESS_NAME);
    if process.is_none() {
        error!("Could not find process {}", PROCESS_NAME);
        return Ok(());
    }

    let syringe = Syringe::for_process(process.unwrap());

    // Inject dll to target process
    info!("Injecting to target process...");
    syringe.inject(dll_path.as_path())?;
    info!("Successfully injected.");

    let (mut stream, addr) = listener.accept()?;
    info!("Connection established with rustadventure {}", addr);

    let mut buf = vec![0u8; 1024];
    let mut stdout = std::io::stdout();

    while let Ok(n) = stream.read(&mut buf[..]) {
        if n == 0 {
            break;
        }

        stdout.write_all(&buf[..n])?;
    }

    warn!("DLL exited from target process!");

    Ok(())
}
