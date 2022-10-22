# HTTP server example

```
rustup target add wasm32-wasi
brew install llvm
curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash
docker compose build
docker compose up
```