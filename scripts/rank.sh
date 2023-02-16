#!/bin/bash

cd tool
echo "$GITHUB_TOKEN"
if [ -e "run.log" ]; then
    rm run.log
fi
cargo run -- $1 > run.log 2>&1
