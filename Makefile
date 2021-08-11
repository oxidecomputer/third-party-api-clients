GITHUB_SPEC = $(CURDIR)/specs/github/api.github.com.json
GITHUB_SPEC_REPO = github/rest-api-description
GITHUB_SPEC_REMOTE = https://raw.githubusercontent.com/$(GITHUB_SPEC_REPO)/main/descriptions/api.github.com/api.github.com.json

GUSTO_SPEC = $(CURDIR)/specs/gusto/gusto.v1.yaml
GUSTO_SPEC_REPO = Gusto-API/api.gusto.dev
GUSTO_SPEC_REMOTE = https://raw.githubusercontent.com/$(GUSTO_SPEC_REPO)/master/reference/Gusto-API.v1.yaml

RAMP_SPEC_DIR = $(CURDIR)/specs/ramp
RAMP_SPEC = $(RAMP_SPEC_DIR)/ramp.v1.json
RAMP_SPEC_REPO = sumatokado/ramp-developer
RAMP_SPEC_REFERENCE = $(RAMP_SPEC_DIR)/reference/Ramp-developer.v1.yaml

generate: github gusto ramp

github: target/debug/generator $(GITHUB_SPEC)
	./target/debug/generator -i $(GITHUB_SPEC) -v 0.1.16 \
		-o github \
		-n octorust \
		--pn GitHub \
		-d "A fully generated & opinionated API client for the GitHub API." \
		--host "api.github.com"
	cargo fmt -p octorust

gusto: target/debug/generator $(GUSTO_SPEC)
	./target/debug/generator -i $(GUSTO_SPEC) -v 0.2.4 \
		-o gusto \
		-n gusto-api \
		--pn Gusto \
		-d "A fully generated & opinionated API client for the Gusto API." \
		--host "api.gusto.com"
	cargo fmt -p gusto-api

$(RAMP_SPEC_REFERENCE):
	git clone git@github.com:$(RAMP_SPEC_REPO).git $(RAMP_SPEC_DIR)

$(RAMP_SPEC): $(RAMP_SPEC_REFERENCE)
	npx swagger-cli bundle \
		--dereference \
		--type json \
		-o $@ $?

ramp: target/debug/generator $(RAMP_SPEC)
	./target/debug/generator -i $(RAMP_SPEC) -v 0.2.0 \
		-o ramp \
		-n ramp-api \
		--pn Ramp \
		-d "A fully generated & opinionated API client for the Ramp API." \
		--host "api.ramp.com"
	cargo fmt -p ramp-api

target/debug/generator: generator/src/*.rs generator/Cargo.toml
	cargo build --bin generator

examples: generate github/examples/*.rs
	cargo build --examples --features="httpcache"
	cargo build --examples
	cargo clippy --examples --features="httpcache"
	cargo clippy --examples

run:
	cargo run --example list_repos_for_org --features="httpcache"

