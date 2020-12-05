YARN?=yarn
YARN_FLAGS?=
CARGO?=cargo
CARGO_FLAGS?=

ifeq ($(APP_ENVIRONMENT),prod)
	ENV=release
	YARN_FLAGS+=--production
	CARGO_FLAGS+=--release
else
	ENV=debug
endif

.DEFAULT_GOAL := build

ifneq (,$(wildcard ./.env))
	include .env
	export
endif

build: web cli
.PHONY: build

web: yarn target/$(ENV)/captainstat-web
.PHONY: web

target/$(ENV)/captainstat-web:
	$(CARGO) build $(CARGO_FLAGS) --package captainstat-web

cli: target/$(ENV)/captainstat-cli
.PHONY: cli

target/$(ENV)/captainstat-cli:
	$(CARGO) build $(CARGO_FLAGS) --bin captainstat-cli

yarn: web/static/lib
.PHONY: yarn

web/static/lib: web/package.json
	cd web && $(YARN) $(YARN_FLAGS) install

serve: web
	microserver web/static/
.PHONY: server
