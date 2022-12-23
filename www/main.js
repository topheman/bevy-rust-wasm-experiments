import './style.css'

import init from './public/out/bevy-rust-wasm-experiments'

document.body.addEventListener('click', () => {
  console.log('click')
  init().then((e) => e).catch(e => e).finally(() => {
    console.log('focus')
    document.querySelector('canvas').focus();
  })
}, { once: true });
