mod bindings;
use bindings::exports::hyperlight_wasm_examples::calculator::calculate::{Guest, Op};

// Bring imported functions from other components into scope
use bindings::hyperlight_wasm_examples::calculator::add::add;
use bindings::hyperlight_wasm_examples::calculator::subtract::subtract;
use bindings::hyperlight_wasm_examples::calculator::multiply::multiply;

struct Component;

impl Guest for Component {
    fn evalexpression(op: Op, x: u32, y: u32) -> u32 {
        match op {
            Op::Add => add(x,y),
            Op::Subtract => subtract(x, y),
            Op::Multiply => multiply(x, y),
        }
    }
}

bindings::export!(Component with_types_in bindings);
