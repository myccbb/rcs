#!/usr/bin/env bash

npx prisma generate --schema prisma/center.prisma

npx tsc -p tsconfig.json

node outDir/server.js
