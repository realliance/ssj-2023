
client:
  cargo run

install-global-deps:
  cargo install wasm-bindgen-cli basic-http-server

serve: client-wasm-opt
  basic-http-server web

client-wasm-opt: client-wasm-bindgen
  wasm-opt -Oz --output web/public/app_bg.wasm web/public/app_bg.wasm

client-wasm-bindgen: build-wasm
  wasm-bindgen --out-name app \
    --out-dir web/public \
    --target web target/wasm32-unknown-unknown/wasm-release/app.wasm

build-wasm:
  cargo build --lib --profile wasm-release --target wasm32-unknown-unknown
