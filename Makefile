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
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

desktop-dev: ## ▶️  Run desktop version in development
	cargo run

desktop-dev-watch: ## ▶️  Run desktop version in development - watch mode
	cargo watch -q -c -x 'run --features bevy/dynamic'

desktop-build: ## ⚙️  Build desktop version
	cargo build --release
	rm -rf ./target/release/assets
	mkdir ./target/release/assets
	cp -r ./assets ./target/release

wasm-dev: ## ▶️  Run wasm version in development mode via wasm-server-runner (useful to work on the WebAssembly bindings)
	@echo "Once started, to access the page with the wasm-bindgen bindings, open http://127.0.0.1:3000/dev.html"
	@echo ""
	WASM_SERVER_RUNNER_ADDRESS=0.0.0.0:3000 cargo run --target wasm32-unknown-unknown

wasm-dev-release: ## ▶️  Run wasm version in development mode via wasm-server-runner (no debug mode - lighter bundle)
	@echo "Once started, to access the page with the wasm-bindgen bindings, open http://127.0.0.1:3000/dev.html"
	@echo ""
	WASM_SERVER_RUNNER_ADDRESS=0.0.0.0:3000 cargo run --release --target wasm32-unknown-unknown

forward-fallback: ## ▶️  forwards port 3000 to localhost.run (to access from mobile on a secure origin)
	ssh -R 80:localhost:3000 localhost.run

forward: ## ▶️  forwards port 3000 to ngrok (to access from mobile on a secure origin)
	@command -v ngrok &> /dev/null && ngrok http 3000 || echo "${_BOLD}ngrok could not be found${_END} - infos to install it are available here: https://ngrok.com\nIf you don't wish to install it, you can use ${_BOLD}make forward-fallback${_END}"

wasm-build: ## ⚙️  Build wasm version
	cargo build --release --target wasm32-unknown-unknown
	wasm-bindgen --out-dir ./www/public/out --target web ./target/wasm32-unknown-unknown/release/bevy-rust-wasm-experiments.wasm

wasm-opt: ## 🔩 Optimize wasm file size
	wasm-opt -Os -o ./www/public/out/bevy-rust-wasm-experiments_bg.wasm ./www/public/out/bevy-rust-wasm-experiments_bg.wasm

wasm-build-opt: ## ⚙️  Build wasm version with optimized file size
	$(MAKE) wasm-build
	$(MAKE) wasm-opt

www-dev: ## ▶️  Build wasm and launch website dev server via vite
	$(MAKE) wasm-build
	$(MAKE) www-dev-only

www-dev-only: ## ▶️  Launch vite dev server (doesn't build wasm)
	cd www && npm run dev -- --host --port 3000

www-build: ## ⚙️  Build wasm and build website
	$(MAKE) wasm-build
	$(MAKE) www-build-only

www-build-only: ## ⚙️  Build vite bundle (doesn't build wasm)
	cd www && npm run build

www-build-opt: ## ⚙️  Build wasm (optimized wasm file size) and build website
	$(MAKE) wasm-build-opt
	$(MAKE) www-build-only

www-preview: ## ▶️  Preview website's build
	cd www && npm run preview -- --host --port 3000

.PHONY: desktop-build desktop-dev-watch desktop-dev forward-fallback forward wasm-build-opt wasm-build wasm-dev-release wasm-dev wasm-opt www-build-only www-build-opt www-build www-dev-only www-dev www-preview
