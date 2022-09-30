#!/usr/bin/env bash

# No Spaces
NAME="$1" 

two_fer(){
    # Must add spaces in the test expression
    if [ "$1" = "" ]; then
        echo "One for you, one for me."
    else 
        echo "One for $1, one for me."
    fi
}

two_fer "$NAME"