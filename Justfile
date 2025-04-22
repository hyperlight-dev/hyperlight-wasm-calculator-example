build-adder:
  cd components/adder && cargo component build --release

init-subtractor:
  cd components/subtractor && npm install 

build-subtractor:
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

