cargo build --release --target wasm32-unknown-unknown

wasm-bindgen --no-typescript --target web --out-dir .\dist --out-name "staff_trainer" .\target\wasm32-unknown-unknown\release\staff_trainer.wasm