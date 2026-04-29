#!/usr/bin/env bash

# Fail on error
set -e

# Set ${SCRIPT_DIR}
SCRIPT_DIR="$(dirname -- "$(readlink -f -- "${0}")")"

for file in "${SCRIPT_DIR}"/colliders/*.webp; do
    OUTPUT="${file%.*}.collision.ron"
    TRIM_W="$(magick "${file}" -trim -format "%w" info:)"
    # NOTE: We are adding 2 because of the outline.
    WIDTH=$((TRIM_W + 2))
    TRIM_H="$(magick "${file}" -trim -format "%h" info:)"
    # NOTE: We are adding 2 because of the outline.
    HEIGHT="$(printf '%s\n' "scale=1; (${TRIM_H} + 2) / 2" | bc)"
    # FIXME: This should (subtract transparent_pixels_on_top / 2) - 1
    OFFSET="$(printf '%s\n' "scale=1; -${HEIGHT} / 2" | bc)"

    printf '%s\n' "Valid shapes are:"
    printf '%s\n' "- ball"
    printf '%s\n' "- capsule"
    printf '%s\n' "- cuboid"
    read -rp "Shape to use for '$(basename "${file}")': " SHAPE

    {
        printf '%s\n' "CollisionData ("
        printf '%s\n' "    shape: Some(\"${SHAPE}\"),"
        printf '%s\n' "    width: Some(${WIDTH}),"
        printf '%s\n' "    height: Some(${HEIGHT}),"
        printf '%s\n' "    y_offset: Some(${OFFSET}),"
        printf '%s\n' ")"
    } >"${OUTPUT}"
done
