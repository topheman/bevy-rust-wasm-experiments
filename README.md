# bevy-rust-experiments

## WebAssembly support

- https://bevy-cheatbook.github.io/platforms/wasm.html
- https://github.com/topheman/rust-wasm-experiments

Add wasm support to your rust installation:

```sh
rustup target install wasm32-unknown-unknown
```

### Development

Install [wasm-server-runner](https://github.com/jakobhellermann/wasm-server-runner)

```sh
cargo install wasm-server-runner
```

Then you can develop targetting the browser:

```sh
cargo run --target wasm32-unknown-unknown
```

### Production

Install [wasm-bindgen-cli](https://rustwasm.github.io/docs/wasm-bindgen/reference/cli.html)

```
cargo install wasm-bindgen-cli
cargo install wasm-opt
```

Then

```
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./www/public/out --target web ./target/wasm32-unknown-unknown/release/bevy-rust-wasm-experiments.wasm
```

You can run the WebAssembly bundle through a vite setup:

```sh
cd www
npm install # only do once
npm run dev
```

The `www/public/assets` is a symlink to `assets` so that the WebAssembly bundle will corectly load assets in browser mode.
