use device_query::{DeviceQuery, DeviceState, Keycode};

pub fn get_key_press(vk: Keycode) -> bool {
    let device_state = DeviceState::new();
    let keys: Vec<Keycode> = device_state.get_keys();

    return keys.contains(&vk);
}
