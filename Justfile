build-adder:
  cd components/adder && cargo component build --release

init-subtractor:
  cd components/subtractor && npm install 

build-subtractor: init-subtractor
  cd components/subtractor && npm run build:component 

build-calculator:
  cd components/calculator && cargo component build --release

build-component: build-adder build-subtractor build-calculator
  # compose the wasm components
  wac plug ./components/calculator/target/wasm32-wasip1/release/calculator.wasm  \
    --plug ./components/adder/target/wasm32-wasip1/release/adder.wasm \
    --plug ./components/subtractor/subtractor.wasm \
    --output calculator-composed.wasm
  # ahead-of-time compile the composed component
  hyperlight-wasm-aot compile --component calculator-composed.wasm ./example/calculator-composed.bin

compile-wit:
  wasm-tools component wit {{ justfile_directory() }}/example/calculator-composed.wit -w -o {{ justfile_directory() }}/example/calculator-composed-world.wasm

build-example: compile-wit
   cd example && \
     HYPERLIGHT_WASM_WORLD={{ justfile_directory() }}/example/calculator-composed-world.wasm \
     cargo build

run-example: compile-wit
   cd example && \
     HYPERLIGHT_WASM_WORLD={{ justfile_directory() }}/example/calculator-composed-world.wasm \
     cargo run