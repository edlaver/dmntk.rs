#!/usr/bin/env bash

# run dmntk as a server
dmntk srv > /dev/null 2>&1 &
_pid=$!
sleep 0.1

# delete all existing models from workspace
curl -s -X POST http://0.0.0.0:22022/definitions/clear > /dev/null 2>&1

# prepare JSON data file containing a model to be added to workspace
echo -n '{"content":"' > data.json
base64 --wrap=0 COMPONENTS.dmn >> data.json
echo -n '"}' >> data.json

# add model to workspace
curl -s -d '@data.json' -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/definitions/add > /dev/null 2>&1

# delete data file
rm data.json

# deploy model
curl -s -X POST http://0.0.0.0:22022/definitions/deploy > /dev/null 2>&1

# evaluate model using generic input data format
echo "=================================="
echo " component"
echo "=================================="
curl -s -d "$(cat input_component.json)" -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/tck/evaluate | jq

echo "=================================="
echo " null component"
echo "=================================="
curl -s -d "$(cat input_null_component.json)" -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/tck/evaluate | jq

echo "=================================="
echo " invalid component (no value)"
echo "=================================="
curl -s -d "$(cat input_invalid_component_no_value.json)" -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/tck/evaluate | jq

echo "=================================="
echo " invalid component (no name)"
echo "=================================="
curl -s -d "$(cat input_invalid_component_no_name.json)" -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/tck/evaluate | jq

# stop dmntk server
kill -s SIGINT "$_pid"
sleep 0.1
