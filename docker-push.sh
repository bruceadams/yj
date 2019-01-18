#!/bin/bash

set -euo pipefail

echo "$DOCKER_PASSWORD" | docker login -u "$DOCKER_USERNAME" --passwd-stdin

set -x

BINARY=yj

docker build --tag "$BINARY:latest" .

docker tag "$BINARY:latest" "$DOCKER_USERNAME/$BINARY:$TRAVIS_TAG"
docker tag "$BINARY:latest" "$DOCKER_USERNAME/$BINARY:latest"

docker push "$DOCKER_USERNAME/$BINARY:$TRAVIS_TAG"
docker push "$DOCKER_USERNAME/$BINARY:latest"
