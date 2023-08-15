# Design document

## Context

Local web page for rending an updated and rendered google earth. It will be streamed in real time for receiving updated maps

## Considered options

- WebGL with Javascript
- Web Assembly for C++ or Rust

## Tasks

- [ ] Image downloader and indexer
- [ ] Add cache and indexer
- [ ] Add logic to update and render
- [ ] wasm draw images

Interesting links:
- https://earth.google.com/web/
- https://github.com/google/earthenterprise
- maplibre-Rust
-
# Design

- Necessary components:
	- PostGRESQL and POSTGIS for keeping
	- TCP Server for getting these images
	- Cache in the frontend side... loading near frames.
	- Set up vector tile clas
	- Call via HTTP a client to receive these images an load them in the web as a grid
