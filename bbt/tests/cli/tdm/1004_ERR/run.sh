#!/usr/bin/env bash

# test DMN model
dmntk tdm -s input.test 2_0001.dmn -i "Greeting Message" 2>&1 | ansifilter -T