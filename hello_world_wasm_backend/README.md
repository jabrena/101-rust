# HTTP server example

```
rustup target add wasm32-wasi
brew install llvm
curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash

docker build -t wasm_backend . 
docker run -d -p 8080:8080 wasm_backend

docker compose build
docker compose up
```

Current issues (22/10/2022):

```
docker compose build
[+] Building 1.3s (14/14) FINISHED                                                                                          
 => [internal] load build definition from Dockerfile                                                                   0.0s
 => => transferring dockerfile: 32B                                                                                    0.0s
 => [internal] load .dockerignore                                                                                      0.0s
 => => transferring context: 2B                                                                                        0.0s
 => [internal] load metadata for docker.io/library/rust:1.64                                                           1.2s
 => [buildbase 1/5] FROM docker.io/library/rust:1.64@sha256:922d814994d77f8e3ab8a7db45a277e9cebe41a557046eeef91a2e34b  0.0s
 => [internal] load build context                                                                                      0.0s
 => => transferring context: 86B                                                                                       0.0s
 => CACHED [buildbase 2/5] RUN rustup target add wasm32-wasi                                                           0.0s
 => CACHED [buildbase 3/5] WORKDIR /src                                                                                0.0s
 => CACHED [buildbase 4/5] COPY src ./src                                                                              0.0s
 => CACHED [buildbase 5/5] COPY Cargo.toml .                                                                           0.0s
 => CACHED [buildserver 1/3] COPY src ./src                                                                            0.0s
 => CACHED [buildserver 2/3] RUN pwd                                                                                   0.0s
 => CACHED [buildserver 3/3] RUN --mount=type=cache,target=/usr/local/cargo/git/db     --mount=type=cache,target=/usr  0.0s
 => CACHED [server 1/1] COPY --from=buildserver /src/target/wasm32-wasi/release/wasmedge_hyper_server.wasm wasmedge_h  0.0s
 => ERROR exporting to image                                                                                           0.0s
 => => exporting layers                                                                                                0.0s
 => => writing image sha256:9811b6619248d3846ca8c70d463c359854511b6539541b40dbfcb799b80e64f2                           0.0s
------
 > exporting to image:
------
failed to solve: operating system is not supported

docker compose up          
[+] Running 0/0
 ⠋ Container hello_world_wasm_backend-server-1  Creating                                                                                                                   0.0s
Error response from daemon: Unknown runtime specified io.containerd.wasmedge.v1
```

Reported issue: https://github.com/WasmEdge/wasmedge_hyper_demo/issues/7

dive wasm_backend   