#!/bin/bash

echo "$GITHUB_TOKEN"
if [ -e "log/run.log" ]; then
    rm log/run.log
fi
cargo run -- $1 > log/run.log 2>&1
