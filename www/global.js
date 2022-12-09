/**
 * Exposed function to rust source code via WebAssembly through wasm-bindgen
 *
 * Those functions will be called from rust source code, search for `#[wasm_bindgen]` directives
 */

function resize_canvas(width, height) {
  console.log(width, height);
  const canvas = document.querySelector('canvas');
  canvas.style.width = `${width}px`;
  canvas.style.height = `${height}px`;
  canvas.width = width * window.devicePixelRatio;
  canvas.height = height * window.devicePixelRatio;
}

// vitejs only accepts <script type="module"/> so we must expose bindings on global scope so they are accessible
window.resize_canvas = resize_canvas;
