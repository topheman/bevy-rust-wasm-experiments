mkdir -p ./www/public/out

touch ./www/public/out/bevy-rust-wasm-experiments_bg.wasm
echo "export default function noop() {}" > ./www/public/out/bevy-rust-wasm-experiments.js
