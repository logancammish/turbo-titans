# turbo-titans

[![Rust](https://github.com/logancammish/turbo-titans/actions/workflows/rust.yml/badge.svg)](https://github.com/logancammish/turbo-titans/actions/workflows/rust.yml)

Car racing game made in Rust. For school. Incomplete.

## Inefficiencies to note:
* The rodio crate (library), specifically this version (`0.17.1`) can see several inefficiencies relating to build and compile times. As an end-user using a binary which has already been compiled, you should not notice any problems.
* In `main.rs`, I implemented a relatively inefficient method to check if a `&str` (string slice) contains an integer value. 
* May face problems with the colored crate in Windows 10.

## Installation: 
Clone this github repo to any directory (`git clone https://github.com/logancammish/turbo-titans.git`)

## Running: 
Windows: Run the batch script found in `src` called `run.bat`.

Linux: you should be able to convert this file (`run.bat`) to the `.sh` extension without issue.
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
