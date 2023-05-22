#!/usr/bin/env bash

# run dmntk as a server
dmntk srv -D . > /dev/null 2>&1 &
_pid=$!
sleep 0.1

# generate a random file with 4MB of data
dd if=/dev/urandom of=random.data bs=3M count=1 > /dev/null 2>&1

# prepare JSON data file containing a model to be added to workspace
echo -n '{"content":"' > data.json
base64 --wrap=0 random.data >> data.json
echo -n '"}' >> data.json

# evaluating the model should fail because of the input data size
curl -s -d '@data.json' -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/evaluate/io.dmntk/compliance-level-2-test-0001/Greeting%20Message

# delete data file
rm data.json
rm random.data

# stop dmntk server
kill -s SIGINT "$_pid"
sleep 0.1
