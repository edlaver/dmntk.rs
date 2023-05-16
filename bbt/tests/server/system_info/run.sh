#!/usr/bin/env bash

# run dmntk as a server
dmntk srv > /dev/null 2>&1 & 
_pid=$!
sleep 0.1

# retrieve system information
curl -s http://0.0.0.0:22022/system/info

# stop dmntk server
kill -s SIGINT "$_pid"
sleep 0.1
