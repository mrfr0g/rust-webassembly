all: clean build copyIndex link

clean:
	rm -rf target/wasm32-unknown-emscripten/release/

build:
	cargo build --release --target wasm32-unknown-emscripten

copyIndex: 
	cp ./index.html target/wasm32-unknown-emscripten/release/

link:
	(cd target/wasm32-unknown-emscripten/release/ ; ln -sf deps/*.wasm ./)