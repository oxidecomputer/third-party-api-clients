GITHUB_SPEC = $(CURDIR)/specs/github/api.github.com.json
RAMP_SPECS = $(CURDIR)/specs/ramp/reference/*.yaml

generate: github

github: target/debug/generator $(GITHUB_SPEC)
	./target/debug/generator -i $(GITHUB_SPEC) -v 0.1.14 \
		-o github \
		-n octorust \
		-d "A fully generated & opinionated API client for the GitHub API."
	cargo fmt

ramp: target/debug/generator $(RAMP_SPEC)
	./target/debug/generator -i $(RAMP_SPEC) -v 0.1.0 \
		-o github \
		-n ramp-api \
		-d "A fully generated & opinionated API client for the Ramp API."
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

