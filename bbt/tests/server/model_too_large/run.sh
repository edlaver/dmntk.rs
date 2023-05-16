#!/usr/bin/env bash

# run dmntk as a server
dmntk srv > /dev/null 2>&1 &
_pid=$!
sleep 0.1

# delete all existing models from workspace
curl -s -X POST http://0.0.0.0:22022/definitions/clear > /dev/null 2>&1

# generate a random file with 4MB of data
dd if=/dev/urandom of=random.data bs=4M count=1 > /dev/null 2>&1

# prepare JSON data file containing a model to be added to workspace
echo -n '{"content":"' > data.json
base64 --wrap=0 random.data >> data.json
echo -n '"}' >> data.json

# add model to workspace
curl -s -d '@data.json' -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/definitions/add

# delete data file
rm random.data
rm data.json

# stop dmntk server
kill -s SIGINT "$_pid"
sleep 0.1
