#!/usr/bin/env bash

# test DMN model
dmntk tdm -s input.test non-existing.dmn -i "Greeting Message" 2>&1 | ansifilter -T
