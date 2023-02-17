#!/bin/bash

# Mode 3: Run Tests
run_output = $(cargo test --format=terse)
test_results = $(echo "$output" | grep "test result")
total_tests = $(echo "$test_results" | awk '{print $4}')
passed_tests = $(echo "$test_results" | awk '{print $3}')
test_percentage = $(echo "($passed_tests / $total_tests) * 100" | bc -1)
coverage_test = $(printf "%.2f $test_percentage)
# Format output
echo "Total Tests: $total_tests" # Total Tests: #
echo "Passed Tests: $passed_tests" # Passed Tests: #
echo "Coverage Test Percentage: $coverage_test" # Coverage Test Percentage: #
# #/## test cases passed. ##% line coverage achieved. 
echo "$passed_tests/$total_tests test cases passed. $coverage_test line coverage achieved."
