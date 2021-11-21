all: native

native:
	cargo build --release

wasm:
	wasm-pack build --release --target nodejs

test:
	cargo test --release

clean:
	cargo clean

.PHONY: all native wasm test clean
