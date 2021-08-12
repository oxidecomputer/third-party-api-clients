DOCUSIGN_SPEC_DIR = $(CURDIR)/specs/docusign
DOCUSIGN_SPEC = $(DOCUSIGN_SPEC_DIR)/docusign.json
DOCUSIGN_SPEC_REPO = docusign/OpenAPI-Specifications
DOCUSIGN_SPEC_REMOTE = https://raw.githubusercontent.com/$(DOCUSIGN_SPEC_REPO)/master/esignature.rest.swagger-v2.1.json

GITHUB_SPEC_DIR = $(CURDIR)/specs/github
GITHUB_SPEC = $(GITHUB_SPEC_DIR)/api.github.com.json
GITHUB_SPEC_REPO = github/rest-api-description
GITHUB_SPEC_REMOTE = https://raw.githubusercontent.com/$(GITHUB_SPEC_REPO)/main/descriptions/api.github.com/api.github.com.json

GUSTO_SPEC_DIR = $(CURDIR)/specs/gusto
GUSTO_SPEC = $(GUSTO_SPEC_DIR)/gusto.v1.yaml
GUSTO_SPEC_REPO = Gusto-API/api.gusto.dev
GUSTO_SPEC_REMOTE = https://raw.githubusercontent.com/$(GUSTO_SPEC_REPO)/master/reference/Gusto-API.v1.yaml

MAILCHIMP_SPEC_DIR = $(CURDIR)/specs/mailchimp
MAILCHIMP_SPEC = $(MAILCHIMP_SPEC_DIR)/mailchimp.json
MAILCHIMP_SPEC_REMOTE = https://api.mailchimp.com/schema/3.0/Swagger.json?expand

OKTA_SPEC_DIR = $(CURDIR)/specs/okta
OKTA_SPEC = $(OKTA_SPEC_DIR)/okta.json
OKTA_SPEC_REPO = okta/okta-management-openapi-spec
OKTA_SPEC_REMOTE = https://raw.githubusercontent.com/$(OKTA_SPEC_REPO)/master/dist/spec.json

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
	$(RM) -r $(DOCUSIGN_SPEC_DIR) $(GITHUB_SPEC_DIR) $(GUSTO_SPEC_DIR) $(MAILCHIMP_SPEC_DIR) $(OKTA_SPEC_DIR) $(ZOOM_SPEC_DIR)
	make $(DOCUSIGN_SPEC) $(GITHUB_SPEC) $(GUSTO_SPEC) $(MAILCHIMP_SPEC) $(OKTA_SPEC) $(ZOOM_SPEC)

$(DOCUSIGN_SPEC_DIR):
	mkdir -p $@

$(DOCUSIGN_SPEC): $(DOCUSIGN_SPEC_DIR)
	npx swagger2openapi \
		--outfile $@ \
		--patch \
		$(DOCUSIGN_SPEC_REMOTE)

docusign: target/debug/generator $(DOCUSIGN_SPEC)
	./target/debug/generator -i $(DOCUSIGN_SPEC) -v 0.2.0 \
		-o docusign \
		-n docusign \
		--proper-name DocuSign \
		-d "A fully generated & opinionated API client for the DocuSign API." \
		--spec-link "https://github.com/$(DOCUSIGN_SPEC_REPO)" \
		--host "na4.docusign.net" \
		--token-endpoint "account.docusign.com/oauth/token" \
		--user-consent-endpoint "account.docusign.com/oauth/auth"
	cargo fmt -p docusign

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

$(MAILCHIMP_SPEC_DIR):
	mkdir -p $@

$(MAILCHIMP_SPEC): $(MAILCHIMP_SPEC_DIR)
	npx swagger2openapi \
		--outfile $@ \
		--patch \
		$(MAILCHIMP_SPEC_REMOTE)

mailchimp: target/debug/generator $(MAILCHIMP_SPEC)
	./target/debug/generator -i $(MAILCHIMP_SPEC) -v 0.2.0 \
		-o mailchimp \
		-n mailchimp-api \
		--proper-name MailChimp \
		-d "A fully generated & opinionated API client for the MailChimp API." \
		--spec-link "$(MAILCHIMP_SPEC_REMOTE)" \
		--host "us1.api.mailchimp.com" \
		--token-endpoint "login.mailchimp.com/oauth2/token" \
		--user-consent-endpoint "login.mailchimp.com/oauth2/authorize"
	cargo fmt -p mailchimp-api

$(OKTA_SPEC_DIR):
	mkdir -p $@

$(OKTA_SPEC): $(OKTA_SPEC_DIR)
	npx swagger2openapi \
		--outfile $@ \
		--patch \
		$(OKTA_SPEC_REMOTE)

okta: target/debug/generator $(OKTA_SPEC)
	./target/debug/generator -i $(OKTA_SPEC) -v 0.2.0 \
		-o okta \
		-n okta \
		--proper-name Okta \
		-d "A fully generated & opinionated API client for the Okta API." \
		--spec-link "https://github.com/$(OKTA_SPEC_REPO)" \
		--host "na4.okta.net" \
		--token-endpoint "account.okta.com/oauth/token" \
		--user-consent-endpoint "account.okta.com/oauth/auth"
	cargo fmt -p okta

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
