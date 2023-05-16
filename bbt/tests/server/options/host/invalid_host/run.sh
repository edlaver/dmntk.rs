#!/usr/bin/env bash

# run dmntk as a server
dmntk srv -H 'invalid host name' 2>&1 &
_pid=$!
sleep 0.1

# stop dmntk server
kill -s SIGINT "$_pid"
sleep 0.1
