use color_eyre::Result;
use device_query::Keycode;
use log::info;

use crate::{hooks, keyboard};

pub mod structs;

pub fn load() -> Result<()> {
    // Initialize hooks
    hooks::initialize()?;

    info!("Waiting for delete key to be pressed");
    loop {
        if keyboard::get_key_press(Keycode::Delete) {
            break;
        }

        std::thread::sleep(std::time::Duration::from_millis(1));
    }

    Ok(())
}

pub fn unload() -> Result<()> {
    // Destroy hooks after loop exits
    hooks::destroy()?;

    Ok(())
}
