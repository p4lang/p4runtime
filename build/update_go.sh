#!/usr/bin/env bash

set -e

THIS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"

pushd $THIS_DIR/.. >/dev/null

docker build -t p4runtime-ci -f build/Dockerfile .

tmpdir=$(mktemp -d -p /tmp)

docker run --rm \
       -v "$tmpdir:/tmp/gen" \
       p4runtime-ci /p4runtime/build/compile_protos.sh /tmp/gen

cp -r $tmpdir/go_out/github.com/p4lang/p4runtime/go/* go/

# Cleanup files owned by root user
docker run --rm \
       -v "$tmpdir:/tmp/gen" \
       p4runtime-ci bash -c "rm -r /tmp/gen/*"

docker run --rm -u "$(id -u):$(id -g)" \
       -e "GOCACHE=/tmp/gocache" \
       -e "GOPATH=/tmp/gopath" \
       -v "$(pwd):/p4runtime" \
       -w /p4runtime \
       golang:1.14 bash -c "go mod tidy"

rm -rf $tmpdir

popd >/dev/null
