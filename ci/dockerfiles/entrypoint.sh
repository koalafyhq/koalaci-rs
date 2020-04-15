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

local start_job=`date +%s`
local koalaci_dir=/opt/koalaci

mkdir -p koalaci_dir

# custom npm/yarn cache dir
mkdir -p /opt/.npm_cache
mkdir -p /opt/.yarn_cache

# TODO: handle nvm/n things
local node_ver=$(node -v)

local deps_cache_dir=.${PROJECT_PACKAGE_MANAGER}_cache
local deps_cache_archive=node-${node_ver}-${PROJECT_PACKAGE_MANAGER}-${PROJECT_ID}.tar.gz

npm config set cache /opt/.npm_cache > /dev/null 2>&1
yarn config set cache-folder /opt/.yarn_cache > /dev/null 2>&1

echo "Getting project"
git clone ${PROJECT_REPO_URL} ${DEPLOYMENT_ID} > /dev/null 2>&1

cd ${DEPLOYMENT_ID}
git checkout ${PROJECT_REPO_BRANCH} > /dev/null 2>&1

echo "Looking up deps cache, package manager using $PROJECT_PACKAGE_MANAGER with node $node_ver"

if test -f "${koalaci_dir}/$deps_cache_archive"; then
  echo "Cache exist, restoring"
  tar -xzf ${koalaci_dir}/${deps_cache_archive} -C /opt/${deps_cache_dir}
else
  echo "Cache not exist"
fi

echo "Installing project dependencies using $PROJECT_PACKAGE_MANAGER"

if [ $PROJECT_PACKAGE_MANAGER == "npm" ]
then
  npm ci
elif [ $PROJECT_PACKAGE_MANAGER == "yarn" ]
then
  yarn install --frozen-lockfile
fi

echo "Building project with command $cmd"

eval "$cmd"

echo "Uploading build artifacts"

cd ${PROJECT_DIST_DIRECTORY}

tar -czf ${koalaci_dir}/${DEPLOYMENT_ID}.tar.gz .

echo "Saving build cache"

cd /opt/${deps_cache_dir}

tar -czf ${koalaci_dir}/${deps_cache_archive} .

local end_job=`date +%s`
local elapsed=$((end_job-start_job))

echo "Done in $(((elapsed % 3600) / 60))m$(((elapsed % 3600) % 60))s"

CODE=$?

exit $CODE