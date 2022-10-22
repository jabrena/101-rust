# Hello World Rust

## Cargo & WASM

```
rustup target add wasm32-unknown-unknown
cargo check --target wasm32-unknown-unknown
```

- https://rustwasm.github.io/wasm-pack/installer/
- https://rustwasm.github.io/book/game-of-life/setup.html
- https://depth-first.com/articles/2020/06/29/compiling-rust-to-webassembly-a-simple-example/

```
cargo build --target wasm32-unknown-unknown --release
wasm-gc target/wasm32-unknown-unknown/release/hello_world_wasm.wasm
```

Running in a web browser:

```
python -m SimpleHTTPServer
open http://localhost:8000/
```

```
cargo install -f cargo-upgrades
```