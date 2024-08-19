# web-assembly-rust

## Example

- [Refereces: Compiling from Rust to WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_Wasm)

```sh
# create project
cargo new --lib hello-wasm

cd hello-wasm

# add wasm-bindgen
cargo add wasm-bindgen

# code

# build package with target option `web`
# other options are `bundler`, `nodejs`, `no-modules`
wasm-pack build --target web

# create index.html
```
