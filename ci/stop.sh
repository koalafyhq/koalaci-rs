#!/bin/bash

set -Eeuo pipefail

trap 'exit' ERR

CONTAINER_NAME=${1}

docker rm koalaci_$CONTAINER_NAME -f

CODE=$?

sleep 3;

exit $CODE;