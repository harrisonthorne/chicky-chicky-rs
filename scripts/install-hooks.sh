#!/bin/bash

GIT_DIR=$(git rev-parse --git-dir)

echo "installing hooks"

ln -srf ./run-tests.sh $GIT_DIR/hooks/pre-commit
ln -srf ./run-tests.sh $GIT_DIR/hooks/pre-push

echo "done!"
