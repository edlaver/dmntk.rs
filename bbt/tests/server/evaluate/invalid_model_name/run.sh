#!/usr/bin/env bash

# run dmntk as a server
dmntk srv -D . > /dev/null 2>&1 & 
_pid=$!
sleep 0.1

# evaluate invocable in model
curl -s -d '{"Full Name":"John Doe"}' -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/evaluate/io.dmntk/compliance-level-2-test-9999/Greeting%20Message

# stop dmntk server
kill -s SIGINT "$_pid"
sleep 0.1
