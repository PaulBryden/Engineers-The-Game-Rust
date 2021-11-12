#Engineers: The Game (Macroquad)
A rewrite of Engineers: The Game, in Rust, using Macroquad.

To build and run on desktop:
```
cargo run --release
```

To build and run for web:
```
cargo build --target wasm32-unknown-unknown --release
cargo install devserver
cp target/wasm32-unknown-unknown/release/engineers.wasm web/engineers.wasm
cd web
devserver
```