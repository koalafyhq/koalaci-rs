#!/bin/bash

set -Eeuo pipefail

trap 'exit' ERR

DOCKER_ENVS=${1}
BUILD_COMMAND=${2}

docker run --rm $DOCKER_ENVS -v ~/koalaci_cache:/tmp koalaci:latest $BUILD_COMMAND
