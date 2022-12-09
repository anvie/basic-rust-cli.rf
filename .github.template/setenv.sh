#!/usr/bin/env bash

# echo "::set-env name=VERSION::$(cat Cargo.toml | grep version | head -n 1 | cut -d '"' -f 2)"

VERSION=$(cat Cargo.toml | grep version | head -n 1 | cut -d '"' -f 2)

echo "VERSION=$VERSION"
echo "BIN_OUTPUT=/usr/local/bin/$name_snake_case$-$VERSION"
