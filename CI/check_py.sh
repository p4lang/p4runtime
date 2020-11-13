#!/usr/bin/env bash

set -eo pipefail

THIS_DIR="$(cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd)"

pushd "$THIS_DIR/.." >/dev/null

rm -rf py/*
./codegen/update_py.sh

diff="$(git status --porcelain py)"

if [ ! -z "$diff" ]; then
    echo "The generated Python files are not up-to-date"
    echo "You can regenerate them with './codegen/update_py.sh' and commit the changes"
    exit 1
fi

popd >/dev/null

