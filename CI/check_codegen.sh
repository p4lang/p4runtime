#!/usr/bin/env bash
# Copyright 2020 Yi Tseng
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
