#!/bin/bash

# Mode 2: Rank Modules
# Second input is the path to a file with a list of URLS
filepath = $2 

# Check if filepath works, if not, print error
if [ ! -f $filepath ]; then 
    echo "Error: filepath $filepath not found"
    exit 1 
fi 

# Read each line of the file and call the necessary files
while read -r line; do
    file = line # Set each file to a line
    # Requirement: our CLI only runs github files, check if github file
    filetype = "github"
    if [[ $file =~ $filetype]]; then 
        # For each file, run the tests and put results in a json output
        testFiles = (calc_responsive_maintainer.rs)
        for file in "${testfiles[@]}"; do 
            ramp_up_score = $(echo "$file" | ./calc_responsive_maintainer.rs)
            # correctness_score = 
            # bus_factor_score =
            # responsive_maintainer_score =
            # license_score =  
            # net_score = 
            # Format output in a json file
            json_output = $(echo "{\"URL": \"$file\", \"NET_SCORE": \"$net_score\",
                                    \"RAMP_UP_SCORE": \"$ramp_up_score\",
                                    \"CORRECTNESS_SCORE": \"$correctness_score\",
                                    \"BUS_FACTOR_SCORE": \"$bus_factor_score\",
                                    \"RESPONSIVE_MAINTAINER_SCORE": \"$responsive_maintainer_score\",
                                    \"LICENSE_SCORE": \"$license_score\"}" | jq .})
            # Output json file to stdout
            echo "$json_output"
        done
    else
        echo "As part of our CLI requirements, we do not process npmjs files."
    fi
done < "$filepath"
