#!/bin/bash

#CITE: I used the examples from this location to help create this script: https://gist.github.com/mbohun/b161521b2440b9f08b59

if [ ${#@} -lt 2 ]; then
    echo "usage: $0 [your github token] [REST expression]"
    exit 1;
fi

GITHUB_TOKEN=$1
GITHUB_API_REST=$2
OUTPUT_FILE=output.txt

GITHUB_API_HEADER_ACCEPT="Accept: application/vnd.github.v3+json"
GITHUB_API__HEADER_AUTHORIZATION="Authorization: Bearer ${GITHUB_TOKEN}" 
GITHUB_API_HEADER_VERSION="X-GitHub-Api-Version: 2022-11-28"

if [ "$GITHUB_API_REST" == "issues" ]; then
    curl -H "$GITHUB_API_HEADER_ACCEPT" -H "$GITHUB_API__HEADER_AUTHORIZATION" -H "$GITHUB_API_HEADER_VERSION" https://api.github.com/repos/PurdueSoftEng/CLI-Tool/issues > $OUTPUT_FILE
elif [ "$GITHUB_API_REST" == "cat" ]; then
    curl -H "$GITHUB_API_HEADER_ACCEPT" -H "$GITHUB_API__HEADER_AUTHORIZATION" -H "$GITHUB_API_HEADER_VERSION" https://api.github.com/octocat > $OUTPUT_FILE
else
    curl -H "$GITHUB_API_HEADER_ACCEPT" -H "$GITHUB_API__HEADER_AUTHORIZATION" -H "$GITHUB_API_HEADER_VERSION" "$GITHUB_API_REST" > $OUTPUT_FILE
fi
