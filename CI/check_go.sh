#!/usr/bin/env bash

set -eo pipefail

THIS_DIR=$(cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd)

pushd $THIS_DIR/.. >/dev/null

rm -rf go/*
./build/update_go.sh

diff="$(git status --porcelain go go.mod go.sum)"

if [ ! -z "$diff" ]; then
    echo "The generated Go files are not up-to-date"
    echo "You can regenerate them with './build/update_go.sh' and commit the changes"
    exit 1
fi

popd >/dev/null

