#!/usr/bin/env bash

# run dmntk as a server
DMNTK_HOST=127.0.0.1 dmntk srv 2>&1 &
_pid=$!
sleep 0.1

# stop dmntk server
kill -s SIGINT "$_pid"
sleep 0.1