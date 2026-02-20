# web-particle

```sh
# =====================================================
# =====================================================
# Ubuntu
# =====================================================
# =====================================================
#   Development: Builds to 'target/debug/'
cd web-particle/
cargo run

# =====================================================
#   Distribution: Builds to 'target/release/'
cargo build --release

# =====================================================
#   Web WASM: Builds to 'target/wasm32-unknown-unknown'
cargo build --release --target wasm32-unknown-unknown

#       Generate the JS Glue Code (run from project root)
wasm-bindgen target/wasm32-unknown-unknown/release/web-particle.wasm \
  --out-dir target/wasm32-unknown-unknown/release \
  --target web \
  --no-typescript

#       Start HTTP server (from project root)
python3 -m http.server 3000
#       Then open: http://localhost:3000/web/index.html

# =====================================================
#   Docker
todo

```


## Requires tools for WASM

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
```