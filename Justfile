build-adder:
  cd components/adder && cargo component build

init-subtractor:
  cd components/subtractor && npm install

build-subtractor:
  cd components/subtractor && npm run build:component

build-calculator:
  cd components/calculator && cargo component build

build-component: build-adder build-subtractor build-calculator
  # Virtualize the calculator component
  wasm-tools print ./components/calculator/target/wasm32-wasip1/debug/calculator.wasm -o calculator.wat
  sed -i 's/@0.2.3/@0.2.1/g' calculator.wat
  wasm-tools parse calculator.wat -o calculator.hack.wasm
  wasi-virt --out calculator.virtualized.wasm calculator.hack.wasm
  # Virtualize the adder component
  wasm-tools print ./components/adder/target/wasm32-wasip1/debug/adder.wasm -o adder.wat
  sed -i 's/@0.2.3/@0.2.1/g' adder.wat
  wasm-tools parse adder.wat -o adder.hack.wasm
  wasi-virt --out adder.virtualized.wasm adder.hack.wasm
  # compose the components
  wac plug calculator.virtualized.wasm  \
    --plug ./adder.virtualized.wasm \
    --plug ./components/subtractor/subtractor.wasm \
    --output calculator-composed.wasm
