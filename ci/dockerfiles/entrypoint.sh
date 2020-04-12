#!/bin/bash

#####################################################################
#
#     As far as I know, the build process is like this:
# 
#     1. Get source code (commonly from Remote repository)
#     2. Install dependencies (cached deps is better, right?)
#     3. Since this is not a CI platform, we skip the `test` stage ;)
#     4. Build the project (incremental build is better)
#     5. Save cache (both for deps & cache artifacts)
#     6. Uplaod artifacts
#
####################################################################

set -Eeuo pipefail

trap 'exit' ERR

cmd=$*

# TODO: Handle for general purpose
mkdir -p /opt/.npm
npm config set cache /opt/.npm

echo "Getting project"
git clone ${PROJECT_REPO_URL} ${DEPLOYMENT_ID} > /dev/null 2>&1

cd ${DEPLOYMENT_ID}
git checkout ${PROJECT_REPO_BRANCH} > /dev/null 2>&1

echo "Downloading build cache"

if test -f "/tmp/${PROJECT_ID}.tar.gz"; then
  echo "Cache exists, extracting"
  tar -xzf /tmp/${PROJECT_ID}.tar.gz -C /opt/.npm
else
  echo "Cache not exists"
fi

echo "Installing project dependencies"

${PROJECT_PACKAGE_MANAGER} ci

echo "Building project with command $cmd"

eval "$cmd"

echo "Uploading build artifacts"

file="${DEPLOYMENT_ID}.tar.gz"

tar -czf ${file} ${PROJECT_DIST_DIRECTORY}

# TODO: change this
bucket="minio"
host="s3-127-0-0-1.nip.io"
s3_key="minioadmin"
s3_secret="minioadmin"

resource="/${bucket}/${file}"
content_type="application/octet-stream"
date=`date -R`
_signature="PUT\n\n${content_type}\n${date}\n${resource}"
signature=`echo -en ${_signature} | openssl sha1 -hmac ${s3_secret} -binary | base64`

# curl -v -X PUT -T "$file" \
#         -H "Date: ${date}" \
#         -H "Content-Type: ${content_type}" \
#         -H "Host: ${host}" \
#         -H "Authorization: AWS ${s3_key}:${signature}" \
#         https://$host${resource}

echo "Saving build cache"

cd /opt/.npm

tar -czf /tmp/${PROJECT_ID}.tar.gz .

CODE=$?

echo "Done"

exit $CODE