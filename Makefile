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

update: update-specs

update-specs:
	$(RM) -r $(GITHUB_SPEC_DIR) $(GUSTO_SPEC_DIR) $(RAMP_SPEC_DIR) $(ZOOM_SPEC_DIR)
	make $(GITHUB_SPEC) $(GUSTO_SPEC) $(RAMP_SPEC) $(ZOOM_SPEC)

$(GITHUB_SPEC_DIR):
	mkdir -p $@

$(GITHUB_SPEC): $(GITHUB_SPEC_DIR)
	curl -sSL $(GITHUB_SPEC_REMOTE) -o $@

github: target/debug/generator $(GITHUB_SPEC)
	./target/debug/generator -i $(GITHUB_SPEC) -v 0.1.22 \
		-o github \
		-n octorust \
		--proper-name GitHub \
		-d "A fully generated & opinionated API client for the GitHub API." \
		--spec-link "https://github.com/$(GITHUB_SPEC_REPO)" \
		--host "api.github.com"
	cargo fmt -p octorust

$(GUSTO_SPEC_DIR):
	mkdir -p $@

$(GUSTO_SPEC): $(GUSTO_SPEC_DIR)
	curl -sSL $(GUSTO_SPEC_REMOTE) -o $@

gusto: target/debug/generator $(GUSTO_SPEC)
	./target/debug/generator -i $(GUSTO_SPEC) -v 0.2.10 \
		-o gusto \
		-n gusto-api \
		--proper-name Gusto \
		-d "A fully generated & opinionated API client for the Gusto API." \
		--spec-link "https://github.com/$(GUSTO_SPEC_REPO)" \
		--host "api.gusto.com" \
		--token-endpoint "api.gusto.com/oauth/token" \
		--user-consent-endpoint "api.gusto.com/oauth/authorize"
	cargo fmt -p gusto-api

$(RAMP_SPEC_REFERENCE):
	git clone git@github.com:$(RAMP_SPEC_REPO).git $(RAMP_SPEC_DIR)

$(RAMP_SPEC): $(RAMP_SPEC_REFERENCE)
	npx swagger-cli bundle \
		--dereference \
		--type json \
		-o $@ $?

ramp: target/debug/generator $(RAMP_SPEC)
	./target/debug/generator -i $(RAMP_SPEC) -v 0.2.3 \
		-o ramp \
		-n ramp-api \
		--proper-name Ramp \
		-d "A fully generated & opinionated API client for the Ramp API." \
		--spec-link "https://github.com/$(RAMP_SPEC_REPO)" \
		--host "api.ramp.com/developer/v1" \
		--token-endpoint "api.ramp.com/v1/public/customer/token" \
		--user-consent-endpoint "app.ramp.com/v1/authorize"
	cargo fmt -p ramp-api

$(ZOOM_SPEC_DIR):
	mkdir -p $@

$(ZOOM_SPEC): $(ZOOM_SPEC_DIR)
	npx swagger2openapi \
		--outfile $@ \
		--patch \
		$(ZOOM_SPEC_REMOTE)

zoom: target/debug/generator $(ZOOM_SPEC)
	./target/debug/generator -i $(ZOOM_SPEC) -v 0.2.0 \
		-o zoom \
		-n zoom-api \
		--proper-name Zoom \
		-d "A fully generated & opinionated API client for the Zoom API." \
		--spec-link "$(ZOOM_SPEC_REMOTE)" \
		--host "api.zoom.us/v2" \
		--token-endpoint "zoom.us/oauth/token" \
		--user-consent-endpoint "zoom.us/oauth/authorize"
	cargo fmt -p zoom-api
