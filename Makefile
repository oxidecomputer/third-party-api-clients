generate: target/debug/generator api.github.com.json
	./target/debug/generator -i ./api.github.com.json -v 0.0.0 -o github -n github

target/debug/generator: generator/src/*.rs generator/Cargo.toml
	cargo build --bin generator
