
#[allow(warnings)]
mod bindings;

use bindings::{component::add::add::add, Guest};

struct Component;

impl Guest for Component {
    fn eval(op: bindings::Op, x: i32, y: i32) -> i32 {
        match op {
            bindings::Op::Add => add(x, y),
            // bindings::Op::Sub => bindings::sub(x, y),
            // bindings::Op::Mul => bindings::mul(x, y),
            // bindings::Op::Div => bindings::div(x, y),
            // bindings::Op::Mod => bindings::mod_(x, y),
        }
    }
}

bindings::export!(Component with_types_in bindings);
