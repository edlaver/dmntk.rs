#!/usr/bin/env bash

DMNTK_HOST='invalid host name' dmntk srv -c never 2>&1 &
_pid=$!
sleep 0.1

kill -s SIGINT "$_pid"
sleep 0.1