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
	cp index.html generated/
	cp service.jpeg generated/

run: build
	cd generated && python -m http.server 8080 && cd ..

test: build
