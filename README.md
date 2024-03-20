# WASM Component Host

## Building and running this example

Build the components with `cargo component`:

```sh
(cd add && cargo component build --release)
(cd sub && cargo component build --release)
(cd calculator && cargo component build --release)
```

Compose the components with `wasm-tools`:

```sh
wasm-tools compose target/wasm32-wasi/release/calculator.wasm -d target/wasm32-wasi/release/add.wasm -d target/wasm32-wasi/release/sub.wasm -o target/wasm32-wasi/release/composed.wasm
```

Update the `wit` dependencies with `wit-deps`:

```sh
(cd host && wit-deps update)
```

Run the host:

```sh
(cd host && cargo run --release add 1 2 ../target/wasm32-wasi/release/composed.wasm)
```
