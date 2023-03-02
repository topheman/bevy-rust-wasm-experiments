import './style.css'

import init from './public/out/bevy-rust-wasm-experiments'
import './components/qrcode-display'
import './components/footer-display'

const qrcodeDisplay = document.querySelector("qrcode-display");
qrcodeDisplay.setAttribute("data", window.location.href);

document.getElementById('start').addEventListener('click', () => {
  console.log('start');
  document.body.classList.add('playing');
  init();
});

document.querySelector('.back-home-page').addEventListener('click', () => {
  console.log('stop');
  window.location.reload(); // todo find an more elegant and performant to cleanup wasm instanciations
})
