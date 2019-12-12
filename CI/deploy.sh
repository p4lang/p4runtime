#!/usr/bin/env bash

# This script is used to deploy the generated P4Runtime specification documents
# to the gh-pages of the repository.

set -e

TARGET_BRANCH="gh-pages"
REMOTE_NAME="public"

if [ "$#" -ne 1 ]; then
    echo "Expected exactly 1 argument, but got $#"
    exit 1
fi

TARGET_DIR="$1"

pushd $TRAVIS_BUILD_DIR

git config user.name "Travis CI"
git config user.email "p4-api@lists.p4.org"

git remote add $REMOTE_NAME https://${GH_TOKEN}@github.com/p4lang/p4runtime.git
git fetch $REMOTE_NAME +$TARGET_BRANCH:$TARGET_BRANCH
git checkout $TARGET_BRANCH
mkdir -p spec
rm -rf spec/$TARGET_DIR
cp -r docs/v1/build spec/$TARGET_DIR

# Amend previous commit and force-push to gh-pages
git add spec
git commit --amend --date=now -m "Publish spec from Travis"
git push -f $REMOTE_NAME $TARGET_BRANCH
