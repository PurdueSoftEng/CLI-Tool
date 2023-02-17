#!/bin/sh

if [ -e "log/test.log" ]; then
    rm log/test.log
fi
cargo test --tests --bin tool > log/test.log 2>&1


if [ $? -eq 0 ]; then
  passed=$(cat log/test.log | grep -o "ok. [0-9]* passed;" | sed 's/ok. \([0-9]*\) passed;/\1/')
  failed=$(cat log/test.log | grep -o "[0-9]* failed;" | sed 's/\([0-9]*\) failed;/\1/')
  total=$(cat log/test.log | grep -o "running [0-9]* tests" | sed 's/running \([0-9]*\) tests/\1/')
  echo "Total Tests: $total"
  echo "Passed Tests: $passed"
  echo "Coverage Test Percentage: unknown"
else
  echo "Testing failed. Check test.log for more information."
fi

