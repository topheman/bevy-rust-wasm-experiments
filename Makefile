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

desktop-dev: ## ▶️  Run desktop version in development
	cargo run

desktop-build: ## ⚙️  Build desktop version
	cargo build --release
	rm -rf ./target/release/assets
	mkdir ./target/release/assets
	cp -r ./assets ./target/release

wasm-dev: ## ▶️  Run wasm version in development mode via wasm-server-runner (useful to work on the WebAssembly bindings)
	@echo "Once started, to access the page with the wasm-bindgen bindings, open http://127.0.0.1:1334/dev.html"
	@echo ""
	cargo run --target wasm32-unknown-unknown

wasm-build: ## ⚙️  Build wasm version
	cargo build --release --target wasm32-unknown-unknown
	wasm-bindgen --out-dir ./www/public/out --target web ./target/wasm32-unknown-unknown/release/bevy-rust-wasm-experiments.wasm

www-dev: ## ⚙️  Build wasm and launch website dev server via vite
	$(MAKE) wasm-build
	cd www && npm run dev

www-build: ## ⚙️  Build wasm and buil website
	$(MAKE) wasm-build
	cd www && npm run build

www-preview: ## ▶️  Preview website's build
	cd www && npm run preview

.PHONY: desktop-build desktop-dev wasm-build wasm-dev www-build www-dev www-preview
