#!/bin/bash
cwd=$(pwd)
node $cwd/mock-api/db.js & cd $cwd/wasm-spa/app && cargo web start --auto-reload && fg