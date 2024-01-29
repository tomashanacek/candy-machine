PWD := $(shell pwd)
BASENAME := $(shell basename $(PWD))

all: build

build:
	docker run --rm -v "$(PWD)":/code \
  	  --mount type=volume,source="$(BASENAME)_cache",target=/code/target \
  	  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry1 \
  	  cosmwasm/workspace-optimizer:0.12.5
schema:
	./scripts/schema.sh

test:
	cargo test
