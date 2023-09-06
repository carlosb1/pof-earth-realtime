SHELL :=/bin/bash
.DEFAULT_GOAL:=help

help:
	@echo 'Usage:'
	@echo '	make <target>'
	@echo ''
	@echo '	Targets:'
	@echo '		help	This help'
	@echo '		build  Build example'
	@echo '		run	Run server in port 8080'
	@echo ' 	test Execute tests for the project'
	@echo ''

#.cargo/ it includes the configuration
build:
	rm -rf generated
	#cargo build --target wasm32-unknown-unknown
	#wasm-pack build --target web
	wasm-pack build --target no-modules
	#cp index.html generated/
	cp service.jpeg pkg/
	cp worker.js pkg/

run: build
	simple-http-server --cors --coep --coop --nocache

test: build
