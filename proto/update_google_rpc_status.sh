#!/usr/bin/env bash
# Copyright 2025 Steffen Smolka
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
# SPDX-License-Identifier: Apache-2.0

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
