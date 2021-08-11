GITHUB_SPEC_DIR = $(CURDIR)/specs/github
GITHUB_SPEC = $(GITHUB_SPEC_DIR)/api.github.com.json
GITHUB_SPEC_REPO = github/rest-api-description
GITHUB_SPEC_REMOTE = https://raw.githubusercontent.com/$(GITHUB_SPEC_REPO)/main/descriptions/api.github.com/api.github.com.json

GUSTO_SPEC_DIR = $(CURDIR)/specs/gusto
GUSTO_SPEC = $(GUSTO_SPEC_DIR)/gusto.v1.yaml
GUSTO_SPEC_REPO = Gusto-API/api.gusto.dev
GUSTO_SPEC_REMOTE = https://raw.githubusercontent.com/$(GUSTO_SPEC_REPO)/master/reference/Gusto-API.v1.yaml

RAMP_SPEC_DIR = $(CURDIR)/specs/ramp
RAMP_SPEC = $(RAMP_SPEC_DIR)/ramp.v1.json
RAMP_SPEC_REPO = sumatokado/ramp-developer
RAMP_SPEC_REFERENCE = $(RAMP_SPEC_DIR)/reference/Ramp-developer.v1.yaml

ZOOM_SPEC_DIR = $(CURDIR)/specs/zoom
ZOOM_SPEC = $(ZOOM_SPEC_DIR)/zoom.json
ZOOM_SPEC_REMOTE = https://marketplace.zoom.us/docs/api-reference/zoom-api/Zoom%20API.oas2.json

generate: github gusto ramp

target/debug/generator: generator/src/*.rs generator/Cargo.toml
	cargo build --bin generator

examples: generate github/examples/*.rs
	cargo build --examples --features="httpcache"
	cargo build --examples
	cargo clippy --examples --features="httpcache"
	cargo clippy --examples

run:
	cargo run --example list_repos_for_org --features="httpcache"

update-specs:
	$(RM) -r $(GITHUB_SPEC_DIR) $(GUSTO_SPEC_DIR) $(RAMP_SPEC_DIR) $(ZOOM_SPEC_DIR)
	make $(GITHUB_SPEC) $(GUSTO_SPEC) $(RAMP_SPEC) $(ZOOM_SPEC)

$(GITHUB_SPEC_DIR):
	mkdir -p $@

$(GITHUB_SPEC): $(GITHUB_SPEC_DIR)
	curl -sSL $(GITHUB_SPEC_REMOTE) -o $@

github: target/debug/generator $(GITHUB_SPEC)
	./target/debug/generator -i $(GITHUB_SPEC) -v 0.1.16 \
		-o github \
		-n octorust \
		--pn GitHub \
		-d "A fully generated & opinionated API client for the GitHub API." \
		--speclink "https://github.com/$(GITHUB_SPEC_REPO)" \
		--host "api.github.com"
	cargo fmt -p octorust

$(GUSTO_SPEC_DIR):
	mkdir -p $@

$(GUSTO_SPEC): $(GUSTO_SPEC_DIR)
	curl -sSL $(GUSTO_SPEC_REMOTE) -o $@

gusto: target/debug/generator $(GUSTO_SPEC)
	./target/debug/generator -i $(GUSTO_SPEC) -v 0.2.4 \
		-o gusto \
		-n gusto-api \
		--pn Gusto \
		-d "A fully generated & opinionated API client for the Gusto API." \
		--speclink "https://github.com/$(GUSTO_SPEC_REPO)" \
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
		--speclink "https://github.com/$(RAMP_SPEC_REPO)" \
		--host "api.ramp.com"
	cargo fmt -p ramp-api

$(ZOOM_SPEC_DIR):
	mkdir -p $@

$(ZOOM_SPEC): $(ZOOM_SPEC_DIR)
	curl -sSL $(ZOOM_SPEC_REMOTE) -o $@
