# Seed Quickstart

**To get started:**
- Clone this repo

- If you don't have Rust and wasm-bindgen installed, [Download it](https://www.rust-lang.org/tools/install), and run the following commands:

`rustup update`

`rustup target add wasm32-unknown-unknown`

`cargo install wasm-bindgen-cli`

 - Run `build.sh` or `build.ps1`, then then start a dev server that supports WASM.
For example, with [Python](https://www.python.org/downloads/) installed, run `python pyserve.py`.
(Linux users may need to run `python3 serve.py`.)

Once you rename the crate in `Cargo.toml` (The `name` field under `[Package]`), make the 
following changes:

- Replace both occurances of `appname`  (`/appname.js` and `/appname.wasm`) in `index.html` with your crate's name.

- Make the same replacement in either `build.sh`, or `build.ps1`, depending on your
operating system. You may delete the other one.

