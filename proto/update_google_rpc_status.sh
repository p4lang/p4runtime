#!/usr/bin/env bash
#
# Updates proto/google/rpc/status.proto with the latest version from the
# official upstream repository.
#
# The file is very stable and hasn't changed in many years, but we enforce
# freshness of this copy using a GitHub Actions workflow just in case.

set -e

STATUS_PROTO_URL="https://raw.githubusercontent.com/googleapis/googleapis/refs/heads/master/google/rpc/status.proto"
THIS_DIR="$(cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd)"

wget "$STATUS_PROTO_URL" -O "$THIS_DIR/google/rpc/status.proto"
