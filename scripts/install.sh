#!/bin/bash

# Mode 1: Install Dependencies 
if [ -e "$filename" ]; then
    rm build.log
fi
cargo build > build.log 2>&1

if [ $? -eq 0 ]; then
  deps=$(grep -o "Compiling" build.log | wc -l)
  echo "Number of dependencies installed: $deps"
else
  echo "Build failed. Check build.log for more information."
fi
