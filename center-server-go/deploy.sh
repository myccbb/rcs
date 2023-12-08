#!/usr/bin/env bash

pm2 stop center-server-go

go build -v center-server-go

pm2 start --name center-server-go ./run.sh
