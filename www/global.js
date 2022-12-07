/**
 * Exposed function to rust source code via WebAssembly through wasm-bindgen
 *
 * Those functions will be called from rust source code, search for `#[wasm_bindgen]` directives
 */

function resize_canvas(width, height) {
  console.log(width, height)
}

// vitejs only accepts <script type="module"/> so we must expose bindings on global scope so they are accessible
window.resize_canvas = resize_canvas;
