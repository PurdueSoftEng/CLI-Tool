#!/bin/bash

# Mode 1: Install Dependencies
git submodule update --init --recursive --quiet > log/git.log 2>&1
if [ -e "log/build.log" ]; then
    rm log/build.log
fi
cargo clean --quiet
cargo build > log/build.log 2>&1

if [ $? -eq 0 ]; then
  deps=$(grep -o "Compiling" log/build.log | wc -l)
  echo "Number of dependencies installed: $deps"
else
  echo "Build failed. Check build.log for more information."
fi
