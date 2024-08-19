# web-assembly-rust

## Example

- [Refereces: Compiling from Rust to WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_Wasm)

```sh
cd crates

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

## Frontend part by Vite with Svelte and Typescript

- preparation wasm for frontend

```sh
# in crates/hello-wasm
# build wasm as bundler which can be installed by npm
wasm-pack build --out-dir public/pkg-bundler --target bundler
```

- create frontend

```sh
# create project
npm create vite@latest

cd wasm-frontend
npm install
npm run dev

# install bundled wasm at hello-wasm
npm i ../hello-wasm/public/pkg-bundler
```

- needs [vite-plugin-wasm](https://www.npmjs.com/package/vite-plugin-wasm)

```sh
npm i vite-plugin-wasm
npm i vite-plugin-top-level-await
```

- modify `vite.config.ts`

```ts
import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [svelte(), wasm(), topLevelAwait()],
});
```
