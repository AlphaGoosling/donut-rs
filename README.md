# Donut-rs
A Rust implementation of the classic donut.c program.

## Overview
Donut-rs is a Rust "port" of the well-known donut.c program, which renders a rotating 3D donut using mathematical equations. This project aims to replicate the same functionality using Rust.

## Features

- Renders a spinning 3D donut using ASCII characters.
- Uses basic mathematical calculations to simulate rotation and projection.

## Building and Running

1. Clone the repository.
2. Use `cargo run` to compile and execute the program.
3. The program will display a spinning donut in the terminal.

## License

MIT License — see `LICENSE` for details.

## Usage
1. Clone the repository: `git clone https://github.com/AlphaGoosling/donut-rs.git`
2. Change into the project directory: `cd donut-rs`
3. Build and run the program: `cargo run`

## Credits
The original donut.c program was created by a1k0n - https://www.a1k0n.net/2011/07/20/donut-math.html . This Rust implementation is based on their work.

## License
Donut-rs is licensed under the MIT License.

## Known Issues
* Their is a bug where the donut disappears for a few frames usually when it is oriented such that its side is facing the camera. I believe it caused by a problem in the math but I do not know. Maybe it is some edge case thing that can be fixed by using quarternions, maybe something else entirely, i do not know
