#!/usr/bin/env bash

dmntk tdm -s input.test non-existing.dmn -i "Greeting Message" 2>&1 | ansifilter -T