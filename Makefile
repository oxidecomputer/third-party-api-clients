generate: target/debug/generator api.github.com.json
	./target/debug/generator -i ./api.github.com.json -v 0.1.0 \
		-o github \
		-n github-api-client \
		-d "A fully generated & opinionated API client for the GitHub API."

	cargo fmt

target/debug/generator: generator/src/*.rs generator/Cargo.toml
	cargo build --bin generator
