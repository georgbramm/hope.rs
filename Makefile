# Compiler configuration
GENERAL_ARGS = --release
FRONTEND_ARGS = $(GENERAL_ARGS) -p hope_cli
BACKEND_ARGS = $(GENERAL_ARGS) -p hope_server
CONTAINER_RUNTIME ?= docker

# Application configuration
define get_config_value
	$(shell sed -ne 's/^$(1).*"\(.*\)"/\1/p' Config.toml)
endef

MONGO_HOST := $(strip $(call get_config_value,host))
MONGO_USERNAME := $(strip $(call get_config_value,username))
MONGO_PASSWORD := $(strip $(call get_config_value,password))
MONGO_DATABASE := $(strip $(call get_config_value,database))

.PHONY: \
	build-doc \
	build-server \
	build-cli \
	coverage \
	clean \
	deploy \
	run-cli \
	run-server \
	run-mongo \
	stop-mongo 

ifndef VERBOSE
.SILENT:
else
GENERAL_ARGS += -v
endif

all: build-server build-cli

build-server:
	cargo build $(BACKEND_ARGS)

build-doc:
	cargo doc --all --no-deps

build-cli:
	cargo build $(FRONTEND_ARGS)

coverage:
	cd backend && cargo kcov
	
clean:
	cd cli && cargo clean 
	cd server && cargo clean
	cd library && cargo clean
	cargo clean  

deploy:   
	# Deploy the frontend
	$(CONTAINER_RUNTIME) pull georgbramm/build-rust:latest
	$(CONTAINER_RUNTIME) run --rm -it -w /deploy -v $(shell pwd):/deploy \
		georgbramm/build-rust:latest \
		cargo web deploy $(FRONTEND_ARGS)
	# Fix applications path to JavaScript file
	sudo chown -R $(USER) target
	# Build the backend
	sudo chown -R 1000:1000 target
	$(CONTAINER_RUNTIME) pull ekidd/rust-musl-builder:1.39.0
	$(CONTAINER_RUNTIME) run --rm -it -v $(shell pwd):/home/rust/src \
		ekidd/rust-musl-builder:1.39.0 \
		cargo build $(BACKEND_ARGS)
	# Create the container image from the executable
	$(CONTAINER_RUNTIME) build --no-cache -t hopeserver .

run-app: run-mongo
	if [ ! "$(shell $(CONTAINER_RUNTIME) ps -q -f name=hopeserver)" ]; then \
		$(CONTAINER_RUNTIME) run --rm \
			--name hopeserver \
			--network="host" \
			-v $(shell pwd)/backend/tls:/tls \
			-v $(shell pwd)/Config.toml:/Config.toml \
			-d hopeserver ;\
	else \
		echo "App already running" ;\
	fi

run-server: run-mongo
	cargo run $(BACKEND_ARGS)

run-cli:
	cargo run $(FRONTEND_ARGS)

run-mongo:
	if [ ! "$(shell $(CONTAINER_RUNTIME) ps -q -f name=mongo)" ]; then \
		$(CONTAINER_RUNTIME) run --rm --name mongo \
			-e POSTGRES_USER=$(PG_USERNAME) \
			-e POSTGRES_PASSWORD=$(PG_PASSWORD) \
			-e POSTGRES_DB=$(PG_DATABASE) \
			-p 5432:5432 \
			-d mongo ;\
		while true; do \
			if pg_isready -qh $(PG_HOST); then break; fi \
		done ;\
		sleep 1; \
		diesel migration run --database-url \
			postgres://$(PG_USERNAME):$(PG_PASSWORD)@$(PG_HOST)/$(PG_DATABASE) ;\
	else \
		echo "Database already running" ;\
	fi

stop-app: stop-mongo
	$(CONTAINER_RUNTIME) stop hopeserver

stop-mongo:
	$(CONTAINER_RUNTIME) stop mongo
