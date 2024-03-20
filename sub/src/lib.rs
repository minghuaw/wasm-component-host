use bindings::exports::component::sub::sub::Guest;

#[allow(warnings)]
mod bindings;

struct Component;

impl Guest for Component {
    fn sub(x: i32, y: i32) -> i32 {
        x - y
    }
}

bindings::export!(Component with_types_in bindings);
