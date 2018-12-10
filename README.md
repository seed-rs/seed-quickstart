# Seed Quickstart

To get started with, clone this repo, and make the following changes:

- Rename your crate in `Cargo.toml` (The `name` field under `[Package]`)

- Replace both occurances of `appname`  (`/appname.js` and `/appname.wasm`) in `index.html` with your
crate's name

- Make the same replacement in either `build.sh`, or `build.ps1`, depending on your
operating system (`.sh` for Linux, `.ps` for Windows). You may delete the other one.


- If you don't have Rust installed, nightly set up, and wasm-bindgen, [Download it](https://www.rust-lang.org/en-US/), and run the following commands:

`rustup update`

`rustup default nightly`

`rustup target add wasm32-unknown-unknown --toolchain nightly`

`cargo +nightly install wasm-bindgen-cli`

 - Run `build.sh` or `build.ps1`, then open `index.html` in a web browser.