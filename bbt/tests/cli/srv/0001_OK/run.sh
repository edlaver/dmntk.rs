#!/usr/bin/env bash

# run the server
dmntk srv -P 11011 -H 127.0.0.1 -D . &
_pid=$!
sleep 0.1

# stop dmntk server
kill -s SIGINT "$_pid"
sleep 0.1


