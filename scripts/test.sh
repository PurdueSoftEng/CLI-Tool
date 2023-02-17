#!/bin/sh

cd tool
cd tool
if [ -e "log/test.log" ]; then
    rm log/test.log
fi
cargo test > log/test.log 2>&1


if [ $? -eq 0 ]; then
  failed=$(grep -o "FAILED" log/test.log | wc -l)
  passed=$(grep -o "PASSED" log/test.log | wc -l)
  total=$failed + $passed
  echo "Total Tests: $total"
  echo "Passed Tests: $passed"
  echo "Coverage Test Percentage: unknown"
else
  echo "Testing failed. Check test.log for more information."
fi

