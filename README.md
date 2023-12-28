# Rustic Boids

My playground to learn rust and a bit of webdev.

# Building

Requires `cargo`, `npm` and `wasm-pack`:
```bash
# Clone the repository
$ git clone https://github.com/meintte/rustic_boids
$ cd rustic_boids

# Compile Rust into WebAssembly
$ cd libs/simulation-wasm
$ wasm-pack build --release

# Start the frontend application
$ cd ../../www
$ npm install
$ npm run start
```
