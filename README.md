# Seed Quickstart

**To get started:**
- Clone this repo

- If you don't have Rust and wasm-bindgen installed, [Download it](https://www.rust-lang.org/tools/install), and run the following commands:

`rustup update`

`rustup target add wasm32-unknown-unknown`

`cargo install wasm-bindgen-cli`

If you run into errors while installing `wasm-bindgen-cli`, you may need to install a C++
build chain. On linux, run `sudo apt install build-essential`. On Windows, download and install
[Visual Studio 2017](https://visualstudio.microsoft.com/downloads/); when asked in the installer,
include the C++ workload.

 - Run `build.sh` or `build.ps1`, then then start a dev server that supports WASM.
For example, with [Python](https://www.python.org/downloads/) installed, run `python serve.py`.
(Linux users may need to run `python3 serve.py`.)

If you run into permission errors on `build.sh`, try this command
to allow executing the file:`chmod +x build.sh`. If you run into persmission errors on `build.ps1`, open Powershell as an administrator, and enter this command: `Set-ExecutionPolicy RemoteSigned`.

Once you rename the crate in `Cargo.toml` (The `name` field under `[Package]`), make the 
following change:

- Replace `appname` with your new name in either `build.sh`, or `build.ps1`, depending on your
operating system.

