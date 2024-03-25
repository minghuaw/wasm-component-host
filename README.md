# WASM Component Host

This is an example showing compiling and composing a wasm component and load the component in a host runtime (wasmtime).
This is developed based on the [host](https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/example-host) example.

## Building and running this example

Build the components with [`cargo component`](https://github.com/bytecodealliance/cargo-component):

```sh
(cd add && cargo component build --release)
(cd sub && cargo component build --release)
(cd calculator && cargo component build --release)
```

Compose the components with [`wasm-tools`](https://github.com/bytecodealliance/wasm-tools):

```sh
wasm-tools compose target/wasm32-wasi/release/calculator.wasm -d target/wasm32-wasi/release/add.wasm -d target/wasm32-wasi/release/sub.wasm -o target/wasm32-wasi/release/composed.wasm
```

Update the `wit` dependencies with [`wit-deps`](https://github.com/bytecodealliance/wit-deps):

```sh
(cd host && wit-deps update)
```

Run the host:

```sh
(cd host && cargo run --release add 1 2 ../target/wasm32-wasi/release/composed.wasm)
```
