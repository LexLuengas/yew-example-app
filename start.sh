#!/bin/bash

if [[ "$OSTYPE" == "linux-gnu" || "$OSTYPE" == "darwin"* ]]; then
    lsof -ti:8000 | xargs kill
    lsof -ti:8001 | xargs kill
fi

script_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
node $script_dir/api/server.js & cd $script_dir/wasm-spa/app && cargo web start --auto-reload && fg