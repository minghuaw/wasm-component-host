use std::path::Path;

use wasmtime::{component::{Component, Linker}, Config, Engine, Store};

#[derive(
    wasmtime::component::ComponentType, wasmtime::component::Lift, wasmtime::component::Lower,
)]
#[component(enum)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Op {
    #[component(name = "add")]
    Add,
    #[component(name = "sub")]
    Sub,
}
impl core::fmt::Debug for Op {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Op::Add => f.debug_tuple("Op::Add").finish(),
            Op::Sub => f.debug_tuple("Op::Sub").finish(),
        }
    }
}

pub fn calculate(path: impl AsRef<Path>, op: Op, x: i32, y: i32) -> anyhow::Result<i32> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    let component = Component::from_file(&engine, path)?;

    // We don't need to add to linker if the component itself is self-contained
    let linker = Linker::new(&engine);

    let mut store = Store::new(
        &engine,
        (),
    );

    // Manually instantiate the component
    let instance = linker.instantiate(&mut store, &component)?;
    let f = instance.get_typed_func::<(Op, i32, i32), (i32, )>(&mut store, "eval")?;
    let (result, ) = f.call(&mut store, (op, x, y))?;
    Ok(result)
}