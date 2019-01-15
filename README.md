# WebAssembly Single-Page Application

A Single-Page Application (SPA) written in Rust with the Yew web framework.

## Run locally

Run `start.sh` to start the API server along with the cargo-web server. To run the server or the app for themselves:
* Server: `node api/server.js`
* SPA: `cd wasm-spa/app/ && cargo web start --auto-reload`