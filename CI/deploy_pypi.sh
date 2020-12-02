#!/usr/bin/env bash
# This script is used to deploy P4Runtime Python package to the pypi.org

THIS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"

pushd "$THIS_DIR/.." >/dev/null

rm -rf dist/*
python3 py/setup.py sdist bdist_wheel
python3 -m twine upload -u __token__ -p ${PYPI_TOKEN} \
                        --non-interactive \
                        --disable-progress-bar \
                        --repository pypi \
                        --verbose \
                        dist/*

popd >/dev/null
