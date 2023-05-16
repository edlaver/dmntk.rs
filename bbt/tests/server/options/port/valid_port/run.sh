#!/usr/bin/env bash

# run the server
dmntk srv -P 22023 2>&1 &
_pid=$!
sleep 0.1

# stop dmntk server
kill -s SIGINT "$_pid"
sleep 0.1


