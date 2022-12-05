import './style.css'

import init from './public/out/bevy-rust-wasm-experiments'

function setup() {
  console.log("setup")
  init().then(module => {
    // we never pass here, cause of
    // Error: Using exceptions for control flow, don't mind me. This isn't actually an error!
    console.log("resolved")
    console.log("module", module)
  }).catch(e => {
    console.log("error")
    console.log(e)
  }).finally(() => {
    document.querySelector('canvas').focus();
  })
  document.body.classList.add('started');
  window.removeEventListener('click', setup);
}

window.addEventListener('click', setup);
