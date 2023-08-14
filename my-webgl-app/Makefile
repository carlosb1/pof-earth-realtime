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

build:
	cargo build --target wasm32-unknown-unknown
	wasm-bindgen target/wasm32-unknown-unknown/debug/my-webgl-app.wasm --out-dir generated --target web

run:
	cd generated && python -m http.server 8080 && cd ..

test:
	@echo "Pending for adding a test"
