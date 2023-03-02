import './style.css'

import init from './public/out/bevy-rust-wasm-experiments'
import './components/footer-display'

document.getElementById('start').addEventListener('click', () => {
  console.log('start');
  document.body.classList.add('playing');
  init();
});
