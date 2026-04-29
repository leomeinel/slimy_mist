#!/usr/bin/env bash

# Fail on error
set -e

# Set "${FILES}" from first argument or return if none are given
FILES=("${@}")
[[ "${#FILES[@]}" -eq 0 ]] && {
    printf '%s\n' "ERROR: Please specify at least one file."
    exit 1
}

# Convert all files to ogg, set medium quality and remove video stream
tmpfile="$(mktemp /tmp/"$(basename "${0}")"-XXXXXX)"
for file in "${FILES[@]}"; do
    OUTPUT="${file%.*}.ogg"
    mv "${file}" "${tmpfile}"
    ffmpeg -i "${tmpfile}" -c:a libvorbis -q:a 5 -vn -map_metadata 0 "${OUTPUT}" ||
        mv "${tmpfile}" "${file}"
done

# Remove ${tmpfile}
rm -f "${tmpfile}"
