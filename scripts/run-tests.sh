#!/bin/bash

echo "stashing unstaged changes first"

git stash -u --keep-index &>/dev/null

echo "running checks..."

cargo clippy --tests
result=$?

git stash pop &>/dev/null

if [ $result -ne 0 ]; then
    echo "check failed! fixed issues before committing or pushing"
    exit 1
fi
