#!/bin/bash
script_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
node $script_dir/mock-api/db.js & cd $script_dir/wasm-spa/app && cargo web start --auto-reload && fg