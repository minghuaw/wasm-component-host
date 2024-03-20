use std::path::Path;

use wasmtime::{component::{Component, Linker}, Config, Engine, Store};

wasmtime::component::bindgen!({
    path: "wit",
    world: "calculator"
});

pub fn calculate(path: impl AsRef<Path>, op: Op, x: i32, y: i32) -> anyhow::Result<i32> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    let component = Component::from_file(&engine, path)?;

    let linker = Linker::new(&engine);

    let mut store = Store::new(
        &engine,
        (),
    );
    let (bindings, _) = Calculator::instantiate(&mut store, &component, &linker)?;

    bindings.call_eval(&mut store, op, x, y)
}