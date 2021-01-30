## Chip-8 emulator

Chip-8 emulator implementation in rust.

[Hardware specifications](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#3.1).

2 emulator frontend are available in this project.

#### Native frontend implementation

Runt with
```bash
cargo run --bin native_frontend --release -- -f <rom_file>
```

#### Wasm frontend implementation

Check the workspace member [README](/wasm_frontend/README.md).
