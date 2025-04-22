#![allow(refining_impl_trait)]

extern crate alloc;

mod bindings;
use bindings::hyperlight_wasm_examples::calculator::calculate::Op;
use bindings::hyperlight_wasm_examples::calculator::Calculate;


mod state;
use state::MyState;

impl bindings::hyperlight_wasm_examples::calculator::Multiply for MyState {
    fn r#multiply(&mut self, r#x: u32, r#y: u32) -> u32 {
        r#x * r#y
    }
}

impl bindings::hyperlight_wasm_examples::calculator::CalculatorImports for MyState {
    type Multiply = MyState;
    fn r#multiply(&mut self) -> impl ::core::borrow::BorrowMut<Self::Multiply> {
        self
    }
}

fn main() {
    let state = MyState::new();
    // Set the maximum allowed resource consumption for the sandbox
    let mut sb: hyperlight_wasm::ProtoWasmSandbox =
        hyperlight_wasm::SandboxBuilder::new()
        .with_guest_input_buffer_size(70000000)
        .with_guest_heap_size(200000000)
        .with_guest_panic_context_buffer_size(10000000)
        .with_guest_stack_size(100000000)
        .with_guest_function_call_max_execution_time_millis(0)
        .build()
        .unwrap();
    // Provide any imports implemented by the host (capturing `state`),
    // returning the resource table used to keep track of host
    // resources handed to the component
    let rt = crate::bindings::register_host_functions(&mut sb, state);
    // Initialise the Wasm engine inside the sandbox
    let sb = sb.load_runtime().unwrap();
    let sb = sb.load_module("calculator-composed.bin").unwrap();
    // Wrap up the sandbox and the resources to get something which
    // we can instantiate and run methods against
    let mut wrapped = bindings::CalculatorSandbox {sb, rt};
    // Create the component instance
    let calc_inst = bindings::hyperlight_wasm_examples::calculator::CalculatorExports::calculate(&mut wrapped);
    // Call methods on the guest component instance
    let r = calc_inst.evalexpression(Op::Subtract, 13, 5);
    println!("13 - 5 = {r}");
    let r = calc_inst.evalexpression(Op::Add, 6, 7);
    println!("6 + 7 = {r}");
    let r = calc_inst.evalexpression(Op::Multiply, 4,3);
    println!("4 * 3 = {r}");
}
