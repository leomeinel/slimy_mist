#!/usr/bin/env bash

# Fail on error
set -e

# Set env variables
SCRIPT_DIR="$(dirname -- "$(readlink -f -- "${0}")")"
BINARY_NAME="$(tomlq -r '.bin.[].name' "${SCRIPT_DIR}"/../Cargo.toml)"

# Build specific build for given argument
if [[ -z "${1}" ]]; then
    cargo build --bin "${BINARY_NAME}" --no-default-features --release -j default
elif [[ "${1}" == "web-release" ]]; then
    rustup target add wasm32-unknown-unknown
    cargo build --bin "${BINARY_NAME}" --no-default-features --target wasm32-unknown-unknown --profile web-release -j default
    ## Optimize binary
    OUTPUT="${SCRIPT_DIR}"/../target/wasm32-unknown-unknown/web-release/"${BINARY_NAME}".wasm
    tmpfile="$(mktemp /tmp/"${BINARY_NAME}"-XXXXXX.wasm)"
    mv "${OUTPUT}" "${tmpfile}"
    wasm-opt -Os -o "${OUTPUT}" "${tmpfile}" --enable-bulk-memory-opt --enable-nontrapping-float-to-int
    rm -f "${tmpfile}"
elif [[ "${1}" == "web-dev" ]]; then
    rustup target add wasm32-unknown-unknown
    cargo build --bin "${BINARY_NAME}" --no-default-features --features dev --target wasm32-unknown-unknown --profile web-dev -j default
else
    cargo build --bin "${BINARY_NAME}" -j default
fi
