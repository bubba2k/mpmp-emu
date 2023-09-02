# MPMP Emulator
TUI emulator and debugger for a custom 16-bit CPU previously designed at HTWK Leipzig "MPMP" course.

## Build
Use rustup (https://www.rust-lang.org/tools/install) or use your systems package manager to install a Rust tool chain. Clone the repo, enter the root directory and do:

    cargo build --release
or

    cargo run --release
    
The executable will be located at `target/release/mpmp-emu`.

## Usage

Make sure to use a sufficiently sized terminal window, the emulator is designed to run in fullscreen mode.

All keybindings are explained inside the application. Press [F1] to open the help screen.

Optionally it is possible to specify an input file as the first command line argument (`mpmp-emu <file>`), otherwise simply load files from inside the application.

This software is designed to work with the masm assembler (https://gitlab.com/moseschmiedel/masm) output and thus files are expected in ASCII hex format (see example below). Note that the hex words must be exactly 5 digits (and thus 20 bits) in length.


    00a58 000a0 01108 00659 
    00106 ffb5a 0007f 00090 
    0068f 00868 00105 00280 
    00105 0068f 00868 00105
    



See the `helpers` directory for examples in assembly code and their machine code counter parts.

