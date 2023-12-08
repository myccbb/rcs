#!/usr/bin/env bash
cd center-web
./deploy.sh
cd ..

cd center-server-go
./deploy.sh
cd ..
