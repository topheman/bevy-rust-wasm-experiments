# bevy-rust-wasm-experiments

<p style="text-align:center;"><img src="www/public/bevy-rust-wasm-experiments-logo-445x380.png" /></p>

Ten years ago, I made a small video game in JavaScript that you can play on your smartphone's browser: [topheman/bombs](https://topheman.github.io/bombs/).

The current project is a proof of concept that aims to demonstrate how to code a video game in rust and compile both to:

- a binary executable on OS desktop such as MacOS, Linux, Windows ...
- a web site, that you could access with any browser (via WebAssembly)

the web version shipping with some features in addition, such as accelerometer support (if you load it on your smartphone), which should integrate seemlessly into the rust source code.

## Previous work

In the last five years I've done a few projects involving rust and WebAssembly:

- [topheman/webassembly-wasi-experiments](https://github.com/topheman/webassembly-wasi-experiments): Discover WebAssembly System Interface (WASI) with C/Rust targetting NodeJS, python, Wasmtime and the browser
- [topheman/rust-wasm-experiments](https://github.com/topheman/rust-wasm-experiments): Discover how to use Rust to generate WebAssembly, called by JavaScript
  - [📺🇫🇷 Utiliser WebAssembly, dès aujourd'hui - ParisJS #86](https://www.youtube.com/watch?v=F3wOfWIFzVc&list=PLWhFHBFsRtquZ6hVXVjXmJ-l51ZXuSBtb)

## Contributing

A [Makefile](Makefile) is available with a list of tasks.

### Prerequisites

- node@>=18
- rust@>=1.67.0

### Setup

```sh
# If you haven't yet, wasm support to your rust installation
rustup target install wasm32-unknown-unknown

# Optional crates for development
cargo install wasm-server-runner # https://github.com/jakobhellermann/wasm-server-runner
cargo install cargo-watch # https://github.com/watchexec/cargo-watch

# Mandatory crates
cargo install wasm-bindgen-cli@0.2.84 # cli for wasm-bindgen implementation shipped in Cargo.toml
cargo install wasm-opt # cli that optimizes wasm payload
```

### Folder organization

```
├── assets (contains the images/fonts used in the app)
├── dev.html (custom endpoint used for when running with wasm-server-runner)
├── src (source code of the rust application)
├── target
└── www (source code of the web part)
    ├── dist
    ├── global.js (bindings exposed to wasm-bindgen, used by both dev.html and www/index.html)
    ├── index.html (endpoint used in the final web server)
    ├── public
    │   ├── assets -> ../../assets (symlink to the asset dir so that they will be picked by the bundler and expose to the browser)
```

### Development

#### Desktop

```sh
cargo run
# `make desktop-dev` is an alias for 👆
```

You can compile in watch mode, thanks to [cargo-watch](https://github.com/watchexec/cargo-watch) and [bevy dynamic linking feature](https://bevy-cheatbook.github.io/setup/bevy-config.html#dynamic-linking):

```sh
cargo watch -q -c -x 'run --features bevy/dynamic'
# `make desktop-dev-watch` is an alias for 👆
```

#### WebAssembly

The following will compile the project in WebAssembly and make it available at http://localhost:3000/dev.html

```sh
WASM_SERVER_RUNNER_ADDRESS=0.0.0.0:3000 cargo run --target wasm32-unknown-unknown
# `make wasm-dev` is an alias for 👆
```

#### Web part

When you need to customize the html/js/css that will end up on the server, you will code in `www`.

The following code will compile the WebAssembly version, generate wasm glue code, build the www artefact and launch a server on http://localhost:3000

```sh
# compile WebAssembly version + generate wasm glue code + build the www artefact + launch a server
make www-build && make www-preview
```

You can launch a dev server for www:

```sh
make www-dev
```

### Production

Same as `make www-build`, but [wasm-opt](https://lib.rs/crates/wasm-opt) is run on the wasm payload to make it lighter.

```sh
make www-build-opt
```

## https

Acceleremeter only works on secure origins, so when you will try to access the app on your smartphone via your local ip (like 192.168.1.1), it won't work, since the domain will be recognized as unsecure.

You'll need to tunnel the app with a utility like [localhost.run](https://localhost.run/) or [ngrok](https://ngrok.com) that will open an ssh tunnel and forward traffic on https. Please run the following one time:

```sh
make forward # with ngrok
make forward-fallback # with localhost.run
```

The public https temporary address will be outputted on your terminal (keep in mind you won't access your website through your local network but through the internet, which can take longer - use that only to test accelerometer on mobile devices).

## Assets

- `assets/ball-steel-no-shadow.png`: https://github.com/topheman/gopher-ball/tree/master/assets/imgs
- `assets/impactMetal_heavy_004.ogg`: https://www.kenney.nl/assets/impact-sounds
- `assets/ThaleahFat.ttf`: https://tinyworlds.itch.io/free-pixel-font-thaleah
- `assets/m6x11.ttf`: https://managore.itch.io/m6x11

## Resources

- [rust book](https://doc.rust-lang.org/stable/book/)
- [bevy cheatbook](https://bevy-cheatbook.github.io)
- [Ball.rs](https://github.com/topheman/rust-wasm-experiments/blob/master/crate/src/ball.rs) / [Ball.js](https://github.com/topheman/Ball.js)
- [jeremychone-channel/rust-invaders](https://github.com/jeremychone-channel/rust-invaders)
- [Bevy Tutorial: Start Menu, Bevy UI, and NPCs](https://www.youtube.com/watch?v=qbeu0Mw1HLY&list=PLT_D88-MTFOOh_S9YifHfo6KETvEmRmYh&index=7)
