#!/usr/bin/env bash

# run dmntk as a server
DMNTK_PORT=22023 dmntk srv --port=22024 2>&1 &
_pid=$!
sleep 0.1

# stop dmntk server
kill -s SIGINT "$_pid"
sleep 0.1