.PHONY: all
all:
	make build
	make pack
	make info

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: protogen
protogen:
	substreams protogen --exclude-paths sf/substreams,google

.PHONY: pack
pack:
	substreams pack

.PHONY: info
info:
	substreams info

.PHONY: run
run:
	substreams run  map_total_supply -e mainnet.eth.streamingfast.io:443

.PHONY: gui
gui:
	substreams gui map_total_supply -e mainnet.eth.streamingfast.io:443
