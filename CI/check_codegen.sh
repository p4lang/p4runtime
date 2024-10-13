#!/usr/bin/env bash

set -eo pipefail

THIS_DIR="$(cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd)"

pushd "$THIS_DIR/.." >/dev/null

rm -rf go/*
rm -rf py/p4
rm -rf rust/src
./codegen/update.sh

diff="$(git status --porcelain go go.mod go.sum)"

if [ ! -z "$diff" ]; then
    echo "The generated Go files are not up-to-date"
    echo "You can regenerate them with './codegen/update.sh' and commit the changes"
    exit 1
fi

diff="$(git status --porcelain py)"

if [ ! -z "$diff" ]; then
    echo "The generated Python files are not up-to-date"
    echo "You can regenerate them with './codegen/update.sh' and commit the changes"
    exit 1
fi

diff="$(git status --porcelain rust)"

if [ ! -z "$diff" ]; then
    echo "The generated Rust files are not up-to-date"
    echo "DIFF:"
    echo "$diff"
    echo "You can regenerate them with './codegen/update.sh' and commit the changes"
    exit 1
fi

popd >/dev/null
