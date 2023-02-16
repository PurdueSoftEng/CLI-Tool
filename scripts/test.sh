#!/bin/sh

cd tool
cd tool
if [ -e "build.log" ]; then
    rm build.log
fi
cargo test > test.log 2>&1


if [ $? -eq 0 ]; then
  failed=$(grep -o "FAILED" test.log | wc -l)
  passed=$(grep -o "PASSED" test.log | wc -l)
  total=$failed + $passed
  echo "Total Tests: $total"
  echo "Passed Tests: $passed"
  echo "Coverage Test Percentage: unknown"
else
  echo "Testing failed. Check test.log for more information."
fi

