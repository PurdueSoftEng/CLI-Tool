#!/bin/bash

cd ../tool
GITHUB_TOKEN=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
echo $GITHUB_TOKEN
if [ -e "run.log" ]; then
    rm run.log
fi
GITHUB_TOKEN=$GITHUB_TOKEN cargo run -- $1 > run.log 2>&1