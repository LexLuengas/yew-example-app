#!/bin/bash
cwd=$(pwd)
node $cwd/mock-api/db.js & cd $cwd/wasm-spa && cargo web start --auto-reload && fg