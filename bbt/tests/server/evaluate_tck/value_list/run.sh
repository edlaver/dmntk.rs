#!/usr/bin/env bash

# run dmntk as a server
dmntk srv > /dev/null 2>&1 & 
_pid=$!
sleep 0.1

# delete all existing models from workspace
curl -s -X POST http://0.0.0.0:22022/definitions/clear > /dev/null 2>&1

# prepare JSON data file containing a model to be added to workspace
echo -n '{"content":"' > data.json
base64 --wrap=0 LISTS.dmn >> data.json
echo -n '"}' >> data.json

# add model to workspace
curl -s -d '@data.json' -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/definitions/add > /dev/null 2>&1

# delete data file
rm data.json

# deploy model
curl -s -X POST http://0.0.0.0:22022/definitions/deploy > /dev/null 2>&1

# evaluate model using generic input data format
echo "=========================="
echo " null string list"
echo "=========================="
curl -s -d "$(cat input_null_string_list.json)" -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/tck/evaluate | jq

echo "=========================="
echo " string list"
echo "=========================="
curl -s -d "$(cat input_string_list.json)" -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/tck/evaluate | jq

echo "=========================="
echo " number list"
echo "=========================="
curl -s -d "$(cat input_number_list.json)" -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/tck/evaluate | jq

echo "=========================="
echo " boolean list"
echo "=========================="
curl -s -d "$(cat input_boolean_list.json)" -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/tck/evaluate | jq

echo "=========================="
echo " date list"
echo "=========================="
curl -s -d "$(cat input_date_list.json)" -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/tck/evaluate | jq

echo "=========================="
echo " date time list"
echo "=========================="
curl -s -d "$(cat input_date_time_list.json)" -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/tck/evaluate | jq

echo "=========================="
echo " time list"
echo "=========================="
curl -s -d "$(cat input_time_list.json)" -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/tck/evaluate | jq

echo "=========================="
echo " year month duration list"
echo "=========================="
curl -s -d "$(cat input_ym_duration_list.json)" -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/tck/evaluate | jq

echo "=========================="
echo " day time duration list"
echo "=========================="
curl -s -d "$(cat input_dt_duration_list.json)" -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/tck/evaluate | jq

echo "=========================="
echo " component list"
echo "=========================="
curl -s -d "$(cat input_component_list.json)" -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/tck/evaluate | jq

echo "=========================="
echo " list list"
echo "=========================="
curl -s -d "$(cat input_list_list.json)" -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/tck/evaluate | jq

echo "=========================="
echo " function list (nulls)"
echo "=========================="
curl -s -d "$(cat input_function_list.json)" -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/tck/evaluate | jq

# stop dmntk server
kill -s SIGINT "$_pid"
sleep 0.1
