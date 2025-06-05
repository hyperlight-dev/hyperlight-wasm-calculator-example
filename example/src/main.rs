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

use std::time::Instant;

fn main() {
    let t0 = Instant::now();
    let state = MyState::new();

    let t1 = Instant::now();
    let mut sb: hyperlight_wasm::ProtoWasmSandbox =
        hyperlight_wasm::SandboxBuilder::new()
        .with_guest_input_buffer_size(70_000_000)
        .with_guest_heap_size(200_000_000)
        .with_guest_panic_context_buffer_size(10_000_000)
        .with_guest_stack_size(100_000_000)
        .with_guest_function_call_max_execution_time_millis(0)
        .build()
        .unwrap();
    let t2 = Instant::now();
    println!("Create ProtoWasmSandbox: {:?}", t2 - t1);

    let rt = crate::bindings::register_host_functions(&mut sb, state);
    let t3 = Instant::now();
    println!("Register host functions: {:?}", t3 - t2);

    let sb = sb.load_runtime().unwrap();
    let t4 = Instant::now();
    println!("Load runtime: {:?}", t4 - t3);

    let sb = sb.load_module("calculator-composed.bin").unwrap();
    let t5 = Instant::now();
    println!("Load module: {:?}", t5 - t4);

    let mut wrapped = bindings::CalculatorSandbox { sb, rt };
    let t6 = Instant::now();
    println!("Create wrapped sandbox: {:?}", t6 - t5);

    let calc_inst = bindings::hyperlight_wasm_examples::calculator::CalculatorExports::calculate(&mut wrapped);
    let t7 = Instant::now();
    println!("Create component instance: {:?}", t7 - t6);

    let r = calc_inst.evalexpression(Op::Subtract, 13, 5);
    let t8 = Instant::now();
    println!("Eval expression Subtract: {:?}, {:?}", t8 - t7, r);
    
    // Time from module load to first call
    println!("Time from module load to first call: {:?}", t8 - t4);

    let r = calc_inst.evalexpression(Op::Add, 6, 7);
    let t9 = Instant::now();
    println!("Eval expression Add: {:?}, {:?}", t9 - t8, r);

    let r = calc_inst.evalexpression(Op::Multiply, 4, 3);
    let t10 = Instant::now();
    println!("Eval expression Multiply: {:?}, {:?}", t10 - t9, r);

    println!("Total time: {:?}", t10 - t0);
}
