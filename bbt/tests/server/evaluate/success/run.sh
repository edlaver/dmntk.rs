#!/usr/bin/env bash

dmntk srv -D . > /dev/null 2>&1 & 
_pid=$!
sleep 0.1

echo "evn"
curl -s -d '{"Full Name":"John Doe"}' -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/evn/io.dmntk.2_0001/Greeting%20Message
curl -s -d '{"Monthly Salary":12000}' -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/evn/io.dmntk.2_0002/Yearly%20Salary

echo -e "\nevi"
curl -s -d '{"Full Name":"John Doe"}' -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/evi/io.dmntk.2_0001/_75b3add2-4d36-4a19-a76c-268b49b2f436
curl -s -d '{"Monthly Salary":12000}' -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/evi/io.dmntk.2_0002/d_YearlySalary

kill -s SIGINT "$_pid"
sleep 0.1