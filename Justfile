build-adder:
  cd components/adder && cargo component build

init-subtractor:
  cd components/subtractor && npm install

build-subtractor:
  cd components/subtractor && npm run build:component
