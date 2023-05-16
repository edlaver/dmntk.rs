#!/usr/bin/env bash

# run dmntk server A
dmntk srv 2>&1 & 
_pid_a=$!
sleep 0.1

# run dmntks server B should fail
dmntk srv 2>&1

# stop dmntk server A
kill -s SIGINT "$_pid_a"
sleep 0.1