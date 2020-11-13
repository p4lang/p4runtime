#!/usr/bin/env bash

set -e

THIS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"

rm -rf py/*

pushd "$THIS_DIR/.." >/dev/null

docker build -t p4runtime-ci -f codegen/Dockerfile .

tmpdir="$(mktemp -d /tmp/p4rt.XXXXXX)"

docker run --rm \
       -v "$tmpdir:/tmp/gen" \
       p4runtime-ci /p4runtime/codegen/compile_protos.sh /tmp/gen

cp -r "$tmpdir"/py_out/p4 py/

# Add __init__.py
find py/p4 -type d -exec touch {}/__init__.py \;

# Cleanup files owned by root user
docker run --rm \
       -v "$tmpdir:/tmp/gen" \
       p4runtime-ci bash -c "rm -r /tmp/gen/*"

rm -rf "$tmpdir"

popd >/dev/null
