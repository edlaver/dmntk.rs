#!/usr/bin/env bash

dmntk srv -D . 2>&1 &
_pid=$!
sleep 0.1

kill -s SIGINT "$_pid"
sleep 0.1