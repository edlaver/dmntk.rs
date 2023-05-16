#!/usr/bin/env bash

# run dmntk as a server
dmntk srv > /dev/null 2>&1 & 
_pid=$!
sleep 0.1

# delete all existing models from workspace
curl -s -X POST http://0.0.0.0:22022/definitions/clear > /dev/null 2>&1

# prepare JSON data file containing a model to be added to workspace
echo -n '{"content":"' > data.json
base64 --wrap=0 VALUES.dmn >> data.json
echo -n '"}' >> data.json

# add model to workspace
curl -s -d '@data.json' -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/definitions/add > /dev/null 2>&1

# delete data file
rm data.json

# deploy model
curl -s -X POST http://0.0.0.0:22022/definitions/deploy > /dev/null 2>&1

# evaluate model using generic input data format
echo "=========================="
echo " xsd:string"
echo "=========================="
curl -s -d "$(cat input_string.json)" -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/tck/evaluate | jq

echo "=========================="
echo " xsd:decimal"
echo "=========================="
curl -s -d "$(cat input_number.json)" -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/tck/evaluate | jq

echo "=========================="
echo " xsd:boolean"
echo "=========================="
curl -s -d "$(cat input_boolean.json)" -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/tck/evaluate | jq

echo "=========================="
echo " xsd:date"
echo "=========================="
curl -s -d "$(cat input_date.json)" -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/tck/evaluate | jq

echo "=========================="
echo " xsd:dateTime"
echo "=========================="
curl -s -d "$(cat input_date_time.json)" -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/tck/evaluate | jq

echo "=========================="
echo " xsd:time"
echo "=========================="
curl -s -d "$(cat input_time.json)" -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/tck/evaluate | jq

echo "=========================="
echo " xsd:duration (ym)"
echo "=========================="
curl -s -d "$(cat input_ym_duration.json)" -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/tck/evaluate | jq

echo "=========================="
echo " xsd:duration (dt)"
echo "=========================="
curl -s -d "$(cat input_dt_duration.json)" -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/tck/evaluate | jq

echo "=========================="
echo " xsd:duration (null)"
echo "=========================="
curl -s -d "$(cat input_null_duration.json)" -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/tck/evaluate | jq

echo "=========================="
echo " invalid input (no value)"
echo "=========================="
curl -s -d "$(cat invalid_input_no_value.json)" -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/tck/evaluate | jq

echo "==============================="
echo " invalid input (invalid value)"
echo "==============================="
curl -s -d "$(cat invalid_input_invalid_value.json)" -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/tck/evaluate | jq

echo "==============================="
echo " invalid input (invalid type)"
echo "==============================="
curl -s -d "$(cat invalid_input_type.json)" -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/tck/evaluate | jq

echo "==============================="
echo " function (expected null)"
echo "==============================="
curl -s -d "$(cat input_function.json)" -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/tck/evaluate | jq

# stop dmntk server
kill -s SIGINT "$_pid"
sleep 0.1
