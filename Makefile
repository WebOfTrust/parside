python:
	@cp Cargo.toml cargo/$@/
	@echo "pyo3 = { version = \"0.18.0\", features = [\"abi3\", \"extension-module\"] }" >> cargo/$@/Cargo.toml
	@echo >> cargo/$@/Cargo.toml
	@cat cargo/$@/Cargo.toml.tail >> cargo/$@/Cargo.toml
	@cd cargo/$@ && cargo build --release --target-dir ../../target/$@
	@mv target/$@/release/libparside.dylib target/$@/release/parside.so

python-shell:
	@cd target/python/release/ && python3

rust:
	@cargo build --release

libs: rust python

clean:
	cargo clean

fix:
	cargo fix
	cargo fmt

preflight:
	cargo fmt --check
	cargo clippy -- -D warnings
	cargo build --release
	cargo test --release
	cargo audit
	cargo tarpaulin
