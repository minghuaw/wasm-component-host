use std::path::Path;

use wasmtime::{component::{Component, Linker}, Config, Engine, Store};

wasmtime::component::bindgen!({
    path: "../calculator/wit",
    world: "calculator"
});

struct CalculatorImpl;

impl CalculatorImports for CalculatorImpl {
    fn add(&mut self, x: i32, y: i32) -> wasmtime::Result<i32> {
        Ok(x + y)
    }

    fn sub(&mut self, x: i32, y: i32) -> wasmtime::Result<i32> {
        Ok(x - y)
    }

    fn mul(&mut self, x: i32, y: i32) -> wasmtime::Result<i32> {
        Ok(x * y)
    }

    fn div(&mut self, x: i32, y: i32) -> wasmtime::Result<i32> {
        Ok(x / y)
    }

    fn mod_(&mut self, x: i32, y: i32) -> wasmtime::Result<i32> {
        Ok(x % y)
    }
}

pub fn calculate(path: impl AsRef<Path>, op: Op, x: i32, y: i32) -> anyhow::Result<i32> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    let component = Component::from_file(&engine, path)?;

    let mut linker = Linker::new(&engine);
    Calculator::add_to_linker(&mut linker, |state: &mut CalculatorImpl| state)?;

    let mut store = Store::new(
        &engine,
        CalculatorImpl,
    );
    let (bindings, _) = Calculator::instantiate(&mut store, &component, &linker)?;

    bindings.call_eval(&mut store, op, x, y)
}