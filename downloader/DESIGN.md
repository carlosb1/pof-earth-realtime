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
