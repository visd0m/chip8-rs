# Chip-8 emulator

Chip-8 emulator implementation in rust.

[Hardware specifications](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#3.1).

Two emulator frontends are available in this project.

### Native frontend implementation

Runt with
```bash
cargo run --bin native_frontend --release -- -f <rom_file>
```

### Wasm frontend implementation

Check the workspace member [README](/wasm_frontend/README.md).


### Roms

Some chip-8 roms are available in the root of the project.

It is possible to download them and test the project with the available roms.