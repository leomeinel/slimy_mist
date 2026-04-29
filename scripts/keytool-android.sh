#!/usr/bin/env bash

# Fail on error
set -e

# Set ${SCRIPT_DIR}
SCRIPT_DIR="$(dirname -- "$(readlink -f -- "${0}")")"

# Create keystores
OUTPUT_DIR="${SCRIPT_DIR}"/.keystores
mkdir -p "${OUTPUT_DIR}"
ALIASES=("dev" "prod-sign" "prod-upload")
for alias in "${ALIASES[@]}"; do
    OUTPUT="${OUTPUT_DIR}"/"${alias}".keystore
    echo "Generating keystore ${alias}"
    keytool -genkeypair -keystore "${OUTPUT}" -storetype pkcs12 -keyalg RSA -keysize 4096 -validity 10000 -alias "${alias}" -dname "CN=Leopold Johannes Meinel, L=Berlin, S=Berlin, C=Germany"
    openssl base64 -A -in "${OUTPUT}" -out "${OUTPUT}".base64.txt
done
