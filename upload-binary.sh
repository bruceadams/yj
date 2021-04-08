#!/bin/sh

if [ -z "$CIRRUS_RELEASE" ]; then
    echo "Not a release. No need to deploy!"
    exit 0
fi

if [ -z "$GITHUB_TOKEN" ]; then
    echo "Please provide GitHub access token via GITHUB_TOKEN environment variable!"
    exit 1
fi

UPLOAD_URL=https://uploads.github.com/repos/$CIRRUS_REPO_FULL_NAME/releases/$CIRRUS_RELEASE/assets
CONTENT_TYPE=application/octet-stream

for path in "$@"; do
    echo "Uploading $path..."
    curl -X POST \
        --header "Accept: application/vnd.github.v3+json" \
        --header "Authorization: token $GITHUB_TOKEN" \
        --header "Content-Type: $CONTENT_TYPE" \
        --data-binary "@$path" \
        "$UPLOAD_URL?name=$(basename "$path")"
done
