# rustadventure

PwnAdventure3 cheat made in Rust, this project's goal is to be a performatic and error-safe project.

What has been implemented so far:

- Detour hooking
  - ClientWorld::Tick (GameLogic.dll)
  - GameAPI::Tick (GameLogic.dll)
  - Player::CanJump (GameLogic.dll)
  - Player::Tick (GameLogic.dll)
  - send (ws2_32.dll)

- Cheats
  - Jump cooldown reset

## Credits

Credit where credit is due

- hazedumper's [memory lib](https://github.com/frk1/hazedumper-rs)
- [retour-rs](https://github.com/Hpmason/retour-rs): a fork of detour-rs that works on latest nightly
- [dll-syringe](https://github.com/OpenByteDev/dll-syringe): a great-looking dll injection library for Rust
- my friends, thank you
