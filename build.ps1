cargo build --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/appname.wasm --no-modules --out-dir ./pkg --out-name package