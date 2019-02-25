# WebAssembly Single-Page Application

A very basic Single-Page Application (SPA) written in Rust with the Yew web framework. It makes requests to the Twitter Search API by a set of user-specified keywords and displays the results in a table.

## Setup

Add a file *api/twitter_config.json* containing some valid Twitter API tokens, e.g.

```json
{
	"consumer_key": "...",
	"consumer_secret": "...",
	"access_token": "...",
	"access_token_secret": "..."
}
```

## Run locally

Simply run `start.sh` to start the API server along with the cargo-web server.

To run the server or the app for themselves:
* Server: `node api/server.js`
* SPA: `cd wasm-spa/app/ && cargo web start --auto-reload`