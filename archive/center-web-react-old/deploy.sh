#!/usr/bin/env bash

yarn install

yarn build

source .env

echo "deploy to '${DEPLOY_PATH}'"

cp -RPp build/* ${DEPLOY_PATH}

