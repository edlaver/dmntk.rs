#!/usr/bin/env bash

dmntk srv -D . > /dev/null 2>&1 & 
_pid=$!
sleep 0.1

echo "evn"
curl -s -d '{"Full Name":"John Doe"}' -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/evn/io.dmntk.invalid/Greeting%20Message

echo -e "\nevi"
curl -s -d '{"Full Name":"John Doe"}' -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/evi/io.dmntk.invalid/_654727a5-6dc8-42c3-94fc-ced55682c9ff

kill -s SIGINT "$_pid"
sleep 0.1