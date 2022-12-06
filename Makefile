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
