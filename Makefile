GITHUB_SPEC = $(CURDIR)/specs/github/api.github.com.json
GUSTO_SPEC = $(CURDIR)/specs/gusto/gusto.v1.yaml
RAMP_SPEC = $(CURDIR)/specs/ramp/reference/*.yaml

generate: github

github: target/debug/generator $(GITHUB_SPEC)
	./target/debug/generator -i $(GITHUB_SPEC) -v 0.1.16 \
		-o github \
		-n octorust \
		-d "A fully generated & opinionated API client for the GitHub API."
	cargo fmt

gusto: target/debug/generator $(GUSTO_SPEC)
	./target/debug/generator -i $(GUSTO_SPEC) -v 0.2.0 \
		-o gusto \
		-n gusto-api \
		-d "A fully generated & opinionated API client for the Gusto API."
	cargo fmt

ramp: target/debug/generator $(RAMP_SPEC)
	./target/debug/generator -i $(RAMP_SPEC) -v 0.2.0 \
		-o ramp \
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

