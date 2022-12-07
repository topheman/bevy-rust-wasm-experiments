_GRAY						= \033[0;30m
_GREEN          = \033[0;32m
_END            = \033[m
_BOLD           = \x1b[1m
_BLUE           = \033[36m
_INFO           = ${_BLUE}${_BOLD}
_INFO_LIGHT     = ${_BLUE}
_SUCCESS        = ${_GREEN}${_BOLD}
_SUCCESS_LIGHT  = ${_GREEN}

help:
	@echo ""
	@echo "${_GRAY}# setup section${_END}"
	@echo "${_GRAY}#${_END}"
	@echo "${_GRAY}# To setup your environment, launch the following from the root of the project${_END}"
	@echo "rustup target install wasm32-unknown-unknown"
	@echo "cargo install wasm-server-runner"
	@echo "cargo install wasm-bindgen-cli"
	@echo "${_GRAY}# If you want to do WebAssembly, install the web part${_END}"
	@echo "cd ./www && npm install"
	@echo ""
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

cargo-run: ## ▶️  Run desktop version in debug mode
	cargo run

cargo-build: ## ⚙️  Build desktop version
	cargo build

cargo-run-wasm: ## ▶️  Run wasm version in debug mode via wasm-server-runner
	@echo "Once started, open http://127.0.0.1:1334/dev.html to access the page with the wasm-bindgen bindings"
	cargo run --target wasm32-unknown-unknown

wasm-build: ## ⚙️  Build WebAssembly
	cargo build --release --target wasm32-unknown-unknown
	wasm-bindgen --out-dir ./www/public/out --target web ./target/wasm32-unknown-unknown/release/bevy-rust-wasm-experiments.wasm

wasm-build-dev: ## ⚙️  Build WebAssembly and launch dev server
	$(MAKE) wasm-build
	cd www && npm run dev
