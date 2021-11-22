all: native

native:
	cargo build --release

wasm:
	sed -i -e "s/rlib/cdylib/g" Cargo.toml
	wasm-pack build --release --target nodejs
	sed -i -e "s/cdylib/rlib/g" Cargo.toml

test:
	cargo test --release

clean:
	cargo clean

.PHONY: all native wasm test clean
