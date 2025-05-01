# `hyperlight-wasm` calculator example

THis is an example of a hyperlight-wasm application that runs a wasm component inside a hyperlight-sandbox that implements a calculator (similar to the component model example [here](https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/tutorial)).

The [calculator](./wit/calculator.wit) itself is comprised of three wasm components:
  - `calculator` - the main component that implements the calculator interface (written in Rust)
  - `adder` - a component that implements the addition operation (written in Rust)
  - `subtractor` - a component that implements the subtraction operation (written in Javascript)
  - Additionally there is a `multiply` operator that is implemented as a hyperlight host function.

The [example](./example/) program loads the composed calculator/wasm component into a hyperlight-sandbox along with a wasm runtime and invokes functions as hyperlight-guest calls.

## Requirements

[![Open in VS Code Container](https://img.shields.io/static/v1?label=Remote%20-%20Containers&message=Open&color=blue&logo=visualstudiocode)](https://vscode.dev/redirect?url=vscode://ms-vscode-remote.remote-containers/cloneInVolume?url=https://github.com/jsturtevant/hyperlight-wasm-calculator-example)

[![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://codespaces.new/jsturtevant/hyperlight-wasm-calculator-example/tree/dev-container)


To build and run this example locally, you need the following tools installed on your system:

- [Rust toolchain](https://www.rust-lang.org/tools/install) including `x86_unknown-none` target
  - https://www.rust-lang.org/tools/install
- [Node.js](https://nodejs.org/en/download) (to build the [subtractor](./components/subtractor/) wasm component)
- [wasm-tools](https://github.com/bytecodealliance/wasm-tools?tab=readme-ov-file#installation)
- [wac](https://github.com/bytecodealliance/wac?tab=readme-ov-file#installation)
- [cargo component](https://github.com/bytecodealliance/cargo-component?tab=readme-ov-file#installation)
- hyperlight-wasm-aot
  - cargo install --git https://github.com/hyperlight-dev/hyperlight-wasm hyperlight-wasm-aot
- [Just](https://github.com/casey/just?tab=readme-ov-file#installation)

## Running the example

- Build the guest component
  ```bash
  just build-component
  ```
- Run the example program
  ```bash
  just run-example
  ```
