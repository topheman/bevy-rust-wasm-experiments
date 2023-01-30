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
cargo install cargo-watch
```

The, you can develop in watch mode, targetting native:

```sh
cargo watch -q -c -x 'run --features bevy/dynamic'
```

Or you can develop targetting the browser:

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

## Assets

- `assets/ball-steel-no-shadow.png`: https://github.com/topheman/gopher-ball/tree/master/assets/imgs
- `assets/impactMetal_heavy_004.ogg`: https://www.kenney.nl/assets/impact-sounds
- `assets/ThaleahFat.ttf`: https://tinyworlds.itch.io/free-pixel-font-thaleah
- `assets/m6x11.ttf`: https://managore.itch.io/m6x11
