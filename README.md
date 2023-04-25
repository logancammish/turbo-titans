# About Rust
Rust is a statically-typed language created in 2008 with the aim of reducing
memory leaks and similar memory problems often found in C programs. Rust is
incredibly efficient programming language and incredibly effective in large-
scale applications (such as the Linux kernel, and Twitters backend).

Personally, I chose Rust for these reasons as well as having the urge to learn a
language I didn't fully understand.

Typically in Rust, you construct structs and use implementations to give them a use,
which I use often. Rust is not necessarily a functional programming language, but does
allow many features commonly found in functional progrmaming languages.
I have utilized indentation commonly found in Rust code.

# Videos:
https://www.youtube.com/watch?v=-8tWNvjd4qI (1km race, does not show NPC functionality)

# turbo-titans

[![Rust](https://github.com/logancammish/turbo-titans/actions/workflows/rust.yml/badge.svg)](https://github.com/logancammish/turbo-titans/actions/workflows/rust.yml)

Car racing game made in Rust. For school. Incomplete.

## Inefficiencies to note:
* The rodio crate (library), specifically this version (`0.17.1`) can see several inefficiencies relating to build and compile times. As an end-user using a binary which has already been compiled, you should not notice any problems.
* In `main.rs`, I implemented a relatively inefficient method to check if a `&str` (string slice) contains an integer value.
* May face problems with the colored crate in Windows 10.

## Installation:
Clone this github repo to any directory (`git clone https://github.com/logancammish/turbo-titans.git`) you can also find the installer under the latest release or [here](https://github.com/logancammish/turbo-titans/files/11116638/CONVER_TO_.BAT_installer.TXT) for windows (please note: Github limits the file types I can upload here, so you need to convert this directly from a txt file to a bat file).

## Running:
Windows: Run the batch script found in `src` called `run.bat`, alternatively check the latest release for `release.zip`, some versions lack this zip file and you may have to just run the batch script.

Linux: you should be able to convert this file (`run.bat`) to the `.sh` extension without major issues.

MacOS: you can compile your own binary using cargo: `cargo build --release`
#### You have to install Rust to run this program
MacOS/Linux: `curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`

Windows: `choco install rust` or find the exe at https://www.rust-lang.org/tools/install



## Dependencies:
* `Rust 1.68.0` - Rust compiler
* `rodio 0.17.1` - Used for audio playback
* `colored 2` - Used for coloured text in output
* `rand latest` - Used to generate random numbers
* `crossterm latest` - Used to detect user inputs
* `whoami 1.1.1` - .
* `serde latest` - Used for reading JSON save files
* `serde_json latest` - "
* `clearscreen latest` - Used to clear the terminal output
