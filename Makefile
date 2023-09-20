.PHONY: all
all:
	make build
	make pack
	make graph
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

.PHONY: graph
graph:
	substreams graph

.PHONY: info
info:
	substreams info

.PHONY: run
run:
	substreams run  map_storage_change -e mainnet.eth.streamingfast.io:443

.PHONY: gui
gui:
	substreams gui map_storage_change -e eth.substreams.pinax.network:9000 -s 1

.PHONY: deploy
deploy:
	graph deploy --studio erc-20
