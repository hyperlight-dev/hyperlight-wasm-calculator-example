FROM ghcr.io/hyperlight-dev/hyperlight-wasm-devcontainer:latest
RUN cargo install wac-cli
RUN cargo install --git https://github.com/hyperlight-dev/hyperlight-wasm hyperlight-wasm-aot
RUN cargo install cargo-component --locked