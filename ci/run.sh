#!/bin/bash

set -Eeuo pipefail

trap 'exit' ERR

DOCKER_ENVS=${1}
DEPLOYMENT_ID=${2}
BUILD_COMMAND=${3}

docker run --rm --name koalaci_$DEPLOYMENT_ID $DOCKER_ENVS -v ~/koalaci_cache:/tmp koalaci:latest $BUILD_COMMAND
