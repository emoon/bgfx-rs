## Web example

This is a "port" of the cubes example running in a browser. The following requirements are needed for the example to work:
* Emscripten https://emscripten.org/docs/getting_started/downloads.html and the env setup correctly
* Rust target wasm32-unknown-emscripten `rustup target add wasm32-unknown-emscripten --release`

## To run

Enter the "web_example" directory
* Build with `cargo build --target=wasm32-unknown-emscripten`
* Copy the `.js` and `.wasm` files from the output to static dir. Such as `cp -v target/wasm32-unknown-emscripten/release/web_example.* static`
* Serve the files in `static` with a web server. If you have python installed you can enter the directory and write `python -m http.server`
* Use a compatible browser and open `http://localhost:8000/index.html`
