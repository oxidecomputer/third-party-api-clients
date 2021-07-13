GITHUB_SPEC = $(CURDIR)/specs/github/api.github.com.json

generate: target/debug/generator $(GITHUB_SPEC)
	./target/debug/generator -i $(GITHUB_SPEC) -v 0.1.4 \
		-o github \
		-n octorust \
		-d "A fully generated & opinionated API client for the GitHub API."
	cargo fmt

target/debug/generator: generator/src/*.rs generator/Cargo.toml
	cargo build --bin generator

examples: generate github/examples/*.rs
	cargo build --examples --features="httpcache"
	cargo build --examples
	cargo clippy --examples --features="httpcache"
	cargo clippy --examples

run:
	cargo run --example list_repos_for_org --features="httpcache"

