SHELL := bash

DOCUSIGN_SPEC_DIR = $(CURDIR)/specs/docusign
DOCUSIGN_SPEC = $(DOCUSIGN_SPEC_DIR)/docusign.yaml
DOCUSIGN_SPEC_REPO = docusign/OpenAPI-Specifications
DOCUSIGN_SPEC_REMOTE = https://raw.githubusercontent.com/APIs-guru/openapi-directory/main/APIs/docusign.net/v2.1/openapi.yaml

GIPHY_SPEC_DIR = $(CURDIR)/specs/giphy
GIPHY_SPEC = $(GIPHY_SPEC_DIR)/giphy.yaml
GIPHY_SPEC_REMOTE = https://raw.githubusercontent.com/APIs-guru/openapi-directory/e12d4e5b76c0b3433337c1a3b9ce4b12e6fb7ce0/APIs/giphy.com/1.0/openapi.yaml

GITHUB_SPEC_DIR = $(CURDIR)/specs/github
GITHUB_SPEC = $(GITHUB_SPEC_DIR)/api.github.com.json
GITHUB_SPEC_REPO = github/rest-api-description
GITHUB_SPEC_REMOTE = https://raw.githubusercontent.com/$(GITHUB_SPEC_REPO)/main/descriptions/api.github.com/api.github.com.json

GOOGLE_SPEC_DIR = $(CURDIR)/specs/google

GOOGLE_ADMIN_SPEC_DIR = $(GOOGLE_SPEC_DIR)/admin
GOOGLE_ADMIN_SPEC = $(GOOGLE_ADMIN_SPEC_DIR)/admin.yaml
GOOGLE_ADMIN_SPEC_REMOTE = https://raw.githubusercontent.com/APIs-guru/openapi-directory/main/APIs/googleapis.com/admin/directory_v1/openapi.yaml

GOOGLE_CALENDAR_SPEC_DIR = $(GOOGLE_SPEC_DIR)/calendar
GOOGLE_CALENDAR_SPEC = $(GOOGLE_CALENDAR_SPEC_DIR)/calendar.yaml
GOOGLE_CALENDAR_SPEC_REMOTE = https://raw.githubusercontent.com/APIs-guru/openapi-directory/main/APIs/googleapis.com/calendar/v3/openapi.yaml

GOOGLE_CLOUD_RESOURCE_MANAGER_SPEC_DIR = $(GOOGLE_SPEC_DIR)/cloud-resource-manager
GOOGLE_CLOUD_RESOURCE_MANAGER_SPEC = $(GOOGLE_CLOUD_RESOURCE_MANAGER_SPEC_DIR)/cloud-resource-manager.yaml
GOOGLE_CLOUD_RESOURCE_MANAGER_SPEC_REMOTE = https://raw.githubusercontent.com/APIs-guru/openapi-directory/main/APIs/googleapis.com/cloudresourcemanager/v2/openapi.yaml

GOOGLE_DRIVE_SPEC_DIR = $(GOOGLE_SPEC_DIR)/drive
GOOGLE_DRIVE_SPEC = $(GOOGLE_DRIVE_SPEC_DIR)/drive.yaml
GOOGLE_DRIVE_SPEC_REMOTE = https://raw.githubusercontent.com/APIs-guru/openapi-directory/main/APIs/googleapis.com/drive/v3/openapi.yaml

GOOGLE_GROUPS_SETTINGS_SPEC_DIR = $(GOOGLE_SPEC_DIR)/groups-settings
GOOGLE_GROUPS_SETTINGS_SPEC = $(GOOGLE_GROUPS_SETTINGS_SPEC_DIR)/groups-settings.yaml
GOOGLE_GROUPS_SETTINGS_SPEC_REMOTE = https://raw.githubusercontent.com/APIs-guru/openapi-directory/main/APIs/googleapis.com/groupssettings/v1/openapi.yaml

GOOGLE_SHEETS_SPEC_DIR = $(GOOGLE_SPEC_DIR)/sheets
GOOGLE_SHEETS_SPEC = $(GOOGLE_SHEETS_SPEC_DIR)/sheets.yaml
GOOGLE_SHEETS_SPEC_REMOTE = https://raw.githubusercontent.com/APIs-guru/openapi-directory/main/APIs/googleapis.com/sheets/v4/openapi.yaml

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

REVAI_SPEC_DIR = $(CURDIR)/specs/rev.ai
REVAI_SPEC = $(REVAI_SPEC_DIR)/rev.ai.yaml
REVAI_SPEC_REMOTE = https://raw.githubusercontent.com/APIs-guru/openapi-directory/main/APIs/rev.ai/v1/openapi.yaml

SENDGRID_SPEC_DIR = $(CURDIR)/specs/sendgrid
SENDGRID_SPEC = $(SENDGRID_SPEC_DIR)/sendgrid.json
SENDGRID_SPEC_REPO = sendgrid/sendgrid-oai
SENDGRID_SPEC_REMOTE = https://raw.githubusercontent.com/$(SENDGRID_SPEC_REPO)/main/oai.json

SHIPBOB_SPEC_DIR = $(CURDIR)/specs/shipbob
SHIPBOB_SPEC = $(SHIPBOB_SPEC_DIR)/shipbob.json
SHIPBOB_SPEC_REMOTE = https://developer.shipbob.com/c196c993-6cf8-4901-84aa-b425f3448df3

SHOPIFY_SPEC_DIR = $(CURDIR)/specs/shopify
SHOPIFY_SPEC = $(SHOPIFY_SPEC_DIR)/shopify.json
SHOPIFY_SPEC_REPO = allengrant/shopify_openapi
SHOPIFY_SPEC_REMOTE = https://raw.githubusercontent.com/$(SHOPIFY_SPEC_REPO)/master/shopify_openapi.json

SLACK_SPEC_DIR = $(CURDIR)/specs/slack
SLACK_SPEC = $(SLACK_SPEC_DIR)/slack.json
SLACK_SPEC_REPO = slackapi/slack-api-specs
SLACK_SPEC_REMOTE = https://raw.githubusercontent.com/$(SLACK_SPEC_REPO)/master/web-api/slack_web_openapi_v2.json

STRIPE_SPEC_DIR = $(CURDIR)/specs/stripe
STRIPE_SPEC = $(STRIPE_SPEC_DIR)/stripe.json
STRIPE_SPEC_REMOTE = https://raw.githubusercontent.com/stripe/openapi/master/openapi/spec3.json

TRIPACTIONS_SPEC_DIR = $(CURDIR)/specs/tripactions
TRIPACTIONS_SPEC = $(TRIPACTIONS_SPEC_DIR)/tripactions.yaml
TRIPACTIONS_SPEC_REMOTE = https://app.tripactions.com/api/public-api.yml

ZOOM_SPEC_DIR = $(CURDIR)/specs/zoom
ZOOM_SPEC = $(ZOOM_SPEC_DIR)/zoom.json
ZOOM_SPEC_REMOTE = https://marketplace.zoom.us/docs/api-reference/zoom-api/Zoom%20API.oas2.json

generate: README.md docusign giphy github google-admin google-calendar google-cloud-resource-manager google-drive google-groups-settings google-sheets gusto mailchimp okta ramp revai sendgrid shipbob shopify slack stripe tripactions zoom
	cargo test tests
	cargo clippy

target/debug/generator: generator/src/*.rs generator/Cargo.toml
	cargo build --bin generator

examples: generate github/examples/*.rs
	cargo build --examples --features="httpcache"
	cargo build --examples
	cargo clippy --examples --features="httpcache"
	cargo clippy --examples

run:
	cargo run --example list_repos_for_org --features="httpcache"
	cargo run --example get_repo_in_org --features="httpcache"
	cargo run --example list_pulls_for_repo --features="httpcache"

update: update-specs

update-specs:
	$(RM) -r $(DOCUSIGN_SPEC_DIR) \
		$(GIPHY_SPEC_DIR) \
		$(GITHUB_SPEC_DIR) \
		$(GOOGLE_ADMIN_SPEC_DIR) \
		$(GOOGLE_CALENDAR_SPEC_DIR) \
		$(GOOGLE_CLOUD_RESOURCE_MANAGER_SPEC_DIR) \
		$(GOOGLE_DRIVE_SPEC_DIR) \
		$(GOOGLE_GROUPS_SETTINGS_SPEC_DIR) \
		$(GUSTO_SPEC_DIR) \
		$(MAILCHIMP_SPEC_DIR) \
		$(OKTA_SPEC_DIR) \
		$(SENDGRID_SPEC_DIR) \
		$(SHIPBOB_SPEC_DIR) \
		$(SHOPIFY_SPEC_DIR) \
		$(SLACK_SPEC_DIR) \
		$(REVAI_SPEC_DIR) \
		$(TRIPACTIONS_SPEC_DIR) \
		$(ZOOM_SPEC_DIR)
	make $(DOCUSIGN_SPEC) \
		$(GIPHY_SPEC) \
		$(GITHUB_SPEC) \
		$(GOOGLE_ADMIN_SPEC) \
		$(GOOGLE_CALENDAR_SPEC) \
		$(GOOGLE_CLOUD_RESOURCE_MANAGER_SPEC) \
		$(GOOGLE_DRIVE_SPEC) \
		$(GOOGLE_GROUPS_SETTINGS_SPEC) \
		$(GUSTO_SPEC) \
		$(MAILCHIMP_SPEC) \
		$(OKTA_SPEC) \
		$(SENDGRID_SPEC) \
		$(SHIPBOB_SPEC) \
		$(SHOPIFY_SPEC) \
		$(SLACK_SPEC) \
		$(REVAI_SPEC_DIR) \
		$(TRIPACTIONS_SPEC) \
		$(ZOOM_SPEC)

$(DOCUSIGN_SPEC_DIR):
	mkdir -p $@

$(DOCUSIGN_SPEC): $(DOCUSIGN_SPEC_DIR)
#	Skip docusign spec download until remote file is fixed
# curl -sSL $(DOCUSIGN_SPEC_REMOTE) -o $@

docusign: target/debug/generator $(DOCUSIGN_SPEC)
	./target/debug/generator -i $(DOCUSIGN_SPEC) -v 0.4.0 \
		-o docusign \
		-n docusign \
		--proper-name DocuSign \
		-d "A fully generated & opinionated API client for the DocuSign API." \
		--spec-link "https://github.com/$(DOCUSIGN_SPEC_REPO)" \
		--host "na4.docusign.net" \
		--token-endpoint "account.docusign.com/oauth/token" \
		--user-consent-endpoint "account.docusign.com/oauth/auth" $(EXTRA_ARGS)
	cargo fmt -p docusign
	@echo -e "- [DocuSign](docusign/) [![docs.rs](https://docs.rs/docusign/badge.svg)](https://docs.rs/docusign)" >> README.md

$(GIPHY_SPEC_DIR):
	mkdir -p $@

$(GIPHY_SPEC): $(GIPHY_SPEC_DIR)
	curl -sSL $(GIPHY_SPEC_REMOTE) -o $@

giphy: target/debug/generator $(GIPHY_SPEC)
	./target/debug/generator -i $(GIPHY_SPEC) -v 0.4.0 \
		-o giphy \
		-n giphy-api \
		--proper-name "Giphy" \
		-d "A fully generated & opinionated API client for the Giphy API." \
		--spec-link "https://github.com/APIs-guru/openapi-directory/tree/main/APIs/giphy.com" \
		--host "api.giphy.com/v1" $(EXTRA_ARGS)
	cargo fmt -p giphy-api
	@echo -e "- [Giphy](giphy/) [![docs.rs](https://docs.rs/giphy-api/badge.svg)](https://docs.rs/giphy-api)" >> README.md

$(GITHUB_SPEC_DIR):
	mkdir -p $@

$(GITHUB_SPEC): $(GITHUB_SPEC_DIR)
	curl -sSL $(GITHUB_SPEC_REMOTE) -o $@

github: target/debug/generator $(GITHUB_SPEC)
	./target/debug/generator -i $(GITHUB_SPEC) -v 0.3.2 \
		-o github \
		-n octorust \
		--proper-name GitHub \
		-d "A fully generated & opinionated API client for the GitHub API." \
		--spec-link "https://github.com/$(GITHUB_SPEC_REPO)" \
		--host "api.github.com" $(EXTRA_ARGS)
	cargo fmt -p octorust
	@echo -e "- [GitHub](github/) [![docs.rs](https://docs.rs/octorust/badge.svg)](https://docs.rs/octorust)" >> README.md

.PHONY: google
google: google-admin google-calendar google-drive google-groups-settings google-sheets
	cargo test tests
	cargo clippy

$(GOOGLE_ADMIN_SPEC_DIR):
	mkdir -p $@

$(GOOGLE_ADMIN_SPEC): $(GOOGLE_ADMIN_SPEC_DIR)
	curl -sSL $(GOOGLE_ADMIN_SPEC_REMOTE) -o $@

google-admin: target/debug/generator $(GOOGLE_ADMIN_SPEC)
	./target/debug/generator -i $(GOOGLE_ADMIN_SPEC) -v 0.6.0 \
		-o google/admin \
		-n gsuite-api \
		--proper-name "Google Admin" \
		-d "A fully generated & opinionated API client for the Google Admin API." \
		--spec-link "https://admin.googleapis.com/$discovery/rest?version=directory_v1" \
		--token-endpoint "oauth2.googleapis.com/token" \
		--user-consent-endpoint "accounts.google.com/o/oauth2/v2/auth" \
		--host "www.googleapis.com" $(EXTRA_ARGS)
	cargo fmt -p gsuite-api
	@echo -e "- [Google Admin](google/admin/) [![docs.rs](https://docs.rs/gsuite-api/badge.svg)](https://docs.rs/gsuite-api)" >> README.md

$(GOOGLE_CALENDAR_SPEC_DIR):
	mkdir -p $@

$(GOOGLE_CALENDAR_SPEC): $(GOOGLE_CALENDAR_SPEC_DIR)
	curl -sSL $(GOOGLE_CALENDAR_SPEC_REMOTE) -o $@

google-calendar: target/debug/generator $(GOOGLE_CALENDAR_SPEC)
	./target/debug/generator -i $(GOOGLE_CALENDAR_SPEC) -v 0.5.0 \
		-o google/calendar \
		-n google-calendar \
		--proper-name "Google Calendar" \
		-d "A fully generated & opinionated API client for the Google Calendar API." \
		--spec-link "https://calendar-json.googleapis.com/$discovery/rest?version=v3" \
		--token-endpoint "oauth2.googleapis.com/token" \
		--user-consent-endpoint "accounts.google.com/o/oauth2/v2/auth" \
		--host "www.googleapis.com/calendar/v3" $(EXTRA_ARGS)
	cargo fmt -p google-calendar
	@echo -e "- [Google Calendar](google/calendar/) [![docs.rs](https://docs.rs/google-calendar/badge.svg)](https://docs.rs/google-calendar)" >> README.md

$(GOOGLE_CLOUD_RESOURCE_MANAGER_SPEC_DIR):
	mkdir -p $@

$(GOOGLE_CLOUD_RESOURCE_MANAGER_SPEC): $(GOOGLE_CLOUD_RESOURCE_MANAGER_SPEC_DIR)
	curl -sSL $(GOOGLE_CLOUD_RESOURCE_MANAGER_SPEC_REMOTE) -o $@

google-cloud-resource-manager: target/debug/generator $(GOOGLE_CLOUD_RESOURCE_MANAGER_SPEC)
	./target/debug/generator -i $(GOOGLE_CLOUD_RESOURCE_MANAGER_SPEC) -v 0.4.0 \
		-o google/cloud-resource-manager \
		-n google-cloud-resource-manager \
		--proper-name "Google Cloud Resource Manager" \
		-d "A fully generated & opinionated API client for the Google Cloud Resource Manager API." \
		--spec-link "https://cloudresourcemanager.googleapis.com/$discovery/rest?version=v2" \
		--token-endpoint "oauth2.googleapis.com/token" \
		--user-consent-endpoint "accounts.google.com/o/oauth2/v2/auth" \
		--host "cloudresourcemanager.googleapis.com/v2" $(EXTRA_ARGS)
	cargo fmt -p google-cloud-resource-manager
	@echo -e "- [Google Cloud Resource Manager](google/cloud-resource-manager/) [![docs.rs](https://docs.rs/google-cloud-resource-manager/badge.svg)](https://docs.rs/google-cloud-resource-manager)" >> README.md

$(GOOGLE_DRIVE_SPEC_DIR):
	mkdir -p $@

$(GOOGLE_DRIVE_SPEC): $(GOOGLE_DRIVE_SPEC_DIR)
	curl -sSL $(GOOGLE_DRIVE_SPEC_REMOTE) -o $@

google-drive: target/debug/generator $(GOOGLE_DRIVE_SPEC)
	./target/debug/generator -i $(GOOGLE_DRIVE_SPEC) -v 0.6.0 \
		-o google/drive \
		-n google-drive \
		--proper-name "Google Drive" \
		-d "A fully generated & opinionated API client for the Google Drive API." \
		--spec-link "https://www.googleapis.com/discovery/v1/apis/drive/v3/rest" \
		--token-endpoint "oauth2.googleapis.com/token" \
		--user-consent-endpoint "accounts.google.com/o/oauth2/v2/auth" \
		--host "www.googleapis.com/drive/v3" $(EXTRA_ARGS)
	cargo fmt -p google-drive
	@echo -e "- [Google Drive](google/drive/) [![docs.rs](https://docs.rs/google-drive/badge.svg)](https://docs.rs/google-drive)" >> README.md

$(GOOGLE_GROUPS_SETTINGS_SPEC_DIR):
	mkdir -p $@

$(GOOGLE_GROUPS_SETTINGS_SPEC): $(GOOGLE_GROUPS_SETTINGS_SPEC_DIR)
	curl -sSL $(GOOGLE_GROUPS_SETTINGS_SPEC_REMOTE) -o $@

google-groups-settings: target/debug/generator $(GOOGLE_GROUPS_SETTINGS_SPEC)
	./target/debug/generator -i $(GOOGLE_GROUPS_SETTINGS_SPEC) -v 0.5.0 \
		-o google/groups-settings \
		-n google-groups-settings \
		--proper-name "Google Groups Settings" \
		-d "A fully generated & opinionated API client for the Google Groups Settings API." \
		--spec-link "https://groupssettings.googleapis.com/$discovery/rest?version=v1" \
		--token-endpoint "oauth2.googleapis.com/token" \
		--user-consent-endpoint "accounts.google.com/o/oauth2/v2/auth" \
		--host "www.googleapis.com/groups/v1/groups" $(EXTRA_ARGS)
	cargo fmt -p google-groups-settings
	@echo -e "- [Google Groups Settings](google/groups-settings/) [![docs.rs](https://docs.rs/google-groups-settings/badge.svg)](https://docs.rs/google-groups-settings)" >> README.md

$(GOOGLE_SHEETS_SPEC_DIR):
	mkdir -p $@

$(GOOGLE_SHEETS_SPEC): $(GOOGLE_SHEETS_SPEC_DIR)
	curl -sSL $(GOOGLE_SHEETS_SPEC_REMOTE) -o $@

google-sheets: target/debug/generator $(GOOGLE_SHEETS_SPEC)
	./target/debug/generator -i $(GOOGLE_SHEETS_SPEC) -v 0.6.0 \
		-o google/sheets \
		-n sheets \
		--proper-name "Google Sheets" \
		-d "A fully generated & opinionated API client for the Google Sheets API." \
		--spec-link "https://sheets.googleapis.com/$discovery/rest?version=v4" \
		--token-endpoint "oauth2.googleapis.com/token" \
		--user-consent-endpoint "accounts.google.com/o/oauth2/v2/auth" \
		--host "sheets.googleapis.com" $(EXTRA_ARGS)
	cargo fmt -p sheets
	@echo -e "- [Google Sheets](google/sheets/) [![docs.rs](https://docs.rs/sheets/badge.svg)](https://docs.rs/sheets)" >> README.md

$(GUSTO_SPEC_DIR):
	mkdir -p $@

$(GUSTO_SPEC): $(GUSTO_SPEC_DIR)
	curl -sSL $(GUSTO_SPEC_REMOTE) -o $@

gusto: target/debug/generator $(GUSTO_SPEC)
	./target/debug/generator -i $(GUSTO_SPEC) -v 0.4.0 \
		-o gusto \
		-n gusto-api \
		--proper-name Gusto \
		-d "A fully generated & opinionated API client for the Gusto API." \
		--spec-link "https://github.com/$(GUSTO_SPEC_REPO)" \
		--host "api.gusto.com" \
		--token-endpoint "api.gusto.com/oauth/token" \
		--user-consent-endpoint "api.gusto.com/oauth/authorize" $(EXTRA_ARGS)
	cargo fmt -p gusto-api
	@echo -e "- [Gusto](gusto/) [![docs.rs](https://docs.rs/gusto-api/badge.svg)](https://docs.rs/gusto-api)" >> README.md

$(MAILCHIMP_SPEC_DIR):
	mkdir -p $@

$(MAILCHIMP_SPEC): $(MAILCHIMP_SPEC_DIR)
	npx swagger2openapi \
		--outfile $@ \
		--patch \
		$(MAILCHIMP_SPEC_REMOTE)

mailchimp: target/debug/generator $(MAILCHIMP_SPEC)
	./target/debug/generator -i $(MAILCHIMP_SPEC) -v 0.4.0 \
		-o mailchimp \
		-n mailchimp-api \
		--proper-name MailChimp \
		-d "A fully generated & opinionated API client for the MailChimp API." \
		--spec-link "$(MAILCHIMP_SPEC_REMOTE)" \
		--host "us1.api.mailchimp.com" \
		--token-endpoint "login.mailchimp.com/oauth2/token" \
		--user-consent-endpoint "login.mailchimp.com/oauth2/authorize" $(EXTRA_ARGS)
	cargo fmt -p mailchimp-api
	@echo -e "- [MailChimp](mailchimp/) [![docs.rs](https://docs.rs/mailchimp-api/badge.svg)](https://docs.rs/mailchimp-api)" >> README.md

$(OKTA_SPEC_DIR):
	mkdir -p $@

$(OKTA_SPEC): $(OKTA_SPEC_DIR)
	npx swagger2openapi \
		--outfile $@ \
		--patch \
		$(OKTA_SPEC_REMOTE)

okta: target/debug/generator $(OKTA_SPEC)
	./target/debug/generator -i $(OKTA_SPEC) -v 0.4.0 \
		-o okta \
		-n okta \
		--proper-name Okta \
		-d "A fully generated & opinionated API client for the Okta API." \
		--spec-link "https://github.com/$(OKTA_SPEC_REPO)" \
		--host "na4.okta.net" \
		--token-endpoint "account.okta.com/oauth/token" \
		--user-consent-endpoint "account.okta.com/oauth/auth" $(EXTRA_ARGS)
	cargo fmt -p okta
	@echo -e "- [Okta](okta/) [![docs.rs](https://docs.rs/okta/badge.svg)](https://docs.rs/okta)" >> README.md

$(RAMP_SPEC_REFERENCE):
	git clone git@github.com:$(RAMP_SPEC_REPO).git $(RAMP_SPEC_DIR)

$(RAMP_SPEC):
	npx swagger-cli bundle \
		--dereference \
		--type json \
		-o $@ $?

ramp: target/debug/generator $(RAMP_SPEC)
	./target/debug/generator -i $(RAMP_SPEC) -v 0.4.0 \
		-o ramp \
		-n ramp-api \
		--proper-name Ramp \
		-d "A fully generated & opinionated API client for the Ramp API." \
		--spec-link "https://github.com/$(RAMP_SPEC_REPO)" \
		--host "api.ramp.com/developer/v1" \
		--token-endpoint "api.ramp.com/v1/public/customer/token" \
		--user-consent-endpoint "app.ramp.com/v1/authorize" $(EXTRA_ARGS)
	cargo fmt -p ramp-api
	@echo -e "- [Ramp](ramp/) [![docs.rs](https://docs.rs/ramp-api/badge.svg)](https://docs.rs/ramp-api)" >> README.md

$(REVAI_SPEC_DIR):
	mkdir -p $@

$(REVAI_SPEC): $(REVAI_SPEC_DIR)
	curl -sSL $(REVAI_SPEC_REMOTE) -o $@

revai: target/debug/generator $(REVAI_SPEC)
	./target/debug/generator -i $(REVAI_SPEC) -v 0.5.0 \
		-o rev.ai \
		-n revai \
		--proper-name "Rev.ai" \
		-d "A fully generated & opinionated API client for the Rev.ai API." \
		--spec-link "$(REVAI_SPEC_REMOTE)" \
		--host "api.rev.ai/speechtotext/v1" $(EXTRA_ARGS)
	cargo fmt -p revai
	@echo -e "- [Rev.ai](rev.ai/) [![docs.rs](https://docs.rs/revai/badge.svg)](https://docs.rs/revai)" >> README.md

$(SENDGRID_SPEC_DIR):
	mkdir -p $@

$(SENDGRID_SPEC): $(SENDGRID_SPEC_DIR)
	npx swagger2openapi \
		--outfile $@ \
		--patch \
		$(SENDGRID_SPEC_REMOTE)

sendgrid: target/debug/generator $(SENDGRID_SPEC)
	./target/debug/generator -i $(SENDGRID_SPEC) -v 0.4.0 \
		-o sendgrid \
		-n sendgrid-api \
		--proper-name SendGrid \
		-d "A fully generated & opinionated API client for the SendGrid API." \
		--spec-link "$(SENDGRID_SPEC_REMOTE)" \
		--host "api.sendgrid.com/v3" $(EXTRA_ARGS)
	cargo fmt -p sendgrid-api
	@echo -e "- [SendGrid](sendgrid/) [![docs.rs](https://docs.rs/sendgrid-api/badge.svg)](https://docs.rs/sendgrid-api)" >> README.md

$(SHIPBOB_SPEC_DIR):
	mkdir -p $@

$(SHIPBOB_SPEC): $(SHIPBOB_SPEC_DIR)
	npx swagger2openapi \
		--outfile $@ \
		--patch \
		$(SHIPBOB_SPEC_DIR)/swagger.json

.PHONY: shipbob
shipbob: target/debug/generator $(SHIPBOB_SPEC)
	./target/debug/generator -i $(SHIPBOB_SPEC) -v 0.3.0 \
		-o shipbob \
		-n shipbob \
		--proper-name "ShipBob" \
		-d "A fully generated & opinionated API client for the ShipBob API." \
		--spec-link "$(SHIPBOB_SPEC_REMOTE)" \
		--host "api.shipbob.com/1.0" $(EXTRA_ARGS)
	cargo fmt -p shipbob
	@echo -e "- [shipbob](shipbob/) [![docs.rs](https://docs.rs/shipbob/badge.svg)](https://docs.rs/shipbob)" >> README.md

$(SHOPIFY_SPEC_DIR):
	mkdir -p $@

$(SHOPIFY_SPEC): $(SHOPIFY_SPEC_DIR)
	curl -sSL $(SHOPIFY_SPEC_REMOTE) -o $@

shopify: target/debug/generator $(SHOPIFY_SPEC)
	./target/debug/generator -i $(SHOPIFY_SPEC) -v 0.3.0 \
		-o shopify \
		-n shopify \
		--proper-name "Shopify" \
		-d "A fully generated & opinionated API client for the Shopify API." \
		--spec-link "$(SHOPIFY_SPEC_REMOTE)" \
		--host "{shop}.myshopify.com/admin/api/2021-07" \
		--token-endpoint "{shop}.myshopify.com/admin/oauth/access_token" \
		--user-consent-endpoint "{shop}.myshopify.com/admin/oauth/authorize" $(EXTRA_ARGS)
	cargo fmt -p shopify
	@echo -e "- [Shopify](shopify/) [![docs.rs](https://docs.rs/shopify/badge.svg)](https://docs.rs/shopify)" >> README.md

$(SLACK_SPEC_DIR):
	mkdir -p $@

$(SLACK_SPEC): $(SLACK_SPEC_DIR)
	npx swagger2openapi \
		--outfile $@ \
		--patch \
		$(SLACK_SPEC_REMOTE)

slack: target/debug/generator $(SLACK_SPEC)
	./target/debug/generator -i $(SLACK_SPEC) -v 0.4.0 \
		-o slack \
		-n slack-chat-api \
		--proper-name Slack \
		-d "A fully generated & opinionated API client for the Slack API." \
		--spec-link "$(SLACK_SPEC_REMOTE)" \
		--host "slack.com/api" \
		--token-endpoint "slack.com/api/oauth.v2.access" \
		--user-consent-endpoint "slack.com/oauth/v2/authorize" $(EXTRA_ARGS)
	cargo fmt -p slack-chat-api
	@echo -e "- [Slack](slack/) [![docs.rs](https://docs.rs/slack-chat-api/badge.svg)](https://docs.rs/slack-chat-api)" >> README.md

$(STRIPE_SPEC_DIR):
	mkdir -p $@

$(STRIPE_SPEC): $(STRIPE_SPEC_DIR)
	npx swagger2openapi \
		--outfile $@ \
		--patch \
		$(STRIPE_SPEC_REMOTE)

stripe: target/debug/generator $(STRIPE_SPEC)
	./target/debug/generator -i $(STRIPE_SPEC) -v 0.4.0 \
		-o stripe \
		-n dolladollabills \
		--proper-name Stripe \
		-d "A fully generated & opinionated API client for the Stripe API." \
		--spec-link "$(STRIPE_SPEC_REMOTE)" \
		--host "api.stripe.com/v1" $(EXTRA_ARGS)
	cargo fmt -p dolladollabills
	@echo -e "- [Stripe](stripe/) [![docs.rs](https://docs.rs/dolladollabills/badge.svg)](https://docs.rs/dolladollabills)" >> README.md

$(TRIPACTIONS_SPEC_DIR):
	mkdir -p $@

$(TRIPACTIONS_SPEC): $(TRIPACTIONS_SPEC_DIR)
	curl -sSL $(TRIPACTIONS_SPEC_REMOTE) -o $@

tripactions: target/debug/generator $(TRIPACTIONS_SPEC)
	./target/debug/generator -i $(TRIPACTIONS_SPEC) -v 0.4.0 \
		-o tripactions \
		-n tripactions \
		--proper-name "TripActions" \
		-d "A fully generated & opinionated API client for the TripActions API." \
		--spec-link "https://app.tripactions.com/api/public/documentation/swagger-ui/index.html?configUrl=/api/public/documentation/api-docs/swagger-config" \
		--host "api.tripactions.com" \
		--token-endpoint "api.tripactions.com/ta-auth/oauth/token" $(EXTRA_ARGS)
	cargo fmt -p tripactions
	@echo -e "- [TripActions](tripactions/) [![docs.rs](https://docs.rs/tripactions/badge.svg)](https://docs.rs/tripactions)" >> README.md

$(ZOOM_SPEC_DIR):
	mkdir -p $@

$(ZOOM_SPEC): $(ZOOM_SPEC_DIR)
	npx swagger2openapi \
		--outfile $@ \
		--patch \
		$(ZOOM_SPEC_REMOTE)

zoom: target/debug/generator $(ZOOM_SPEC)
	./target/debug/generator -i $(ZOOM_SPEC) -v 0.4.0 \
		-o zoom \
		-n zoom-api \
		--proper-name Zoom \
		-d "A fully generated & opinionated API client for the Zoom API." \
		--spec-link "$(ZOOM_SPEC_REMOTE)" \
		--host "api.zoom.us/v2" \
		--token-endpoint "zoom.us/oauth/token" \
		--user-consent-endpoint "zoom.us/oauth/authorize" $(EXTRA_ARGS)
	cargo fmt -p zoom-api
	@echo -e "- [Zoom](zoom/) [![docs.rs](https://docs.rs/zoom-api/badge.svg)](https://docs.rs/zoom-api)" >> README.md

.PHONY: README.md
README.md: ## Cleans client info in README.md.
	@sed -i '/## Clients Generated/q' $@
