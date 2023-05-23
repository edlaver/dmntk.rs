#!/usr/bin/env bash

dmntk tdm -s non-existing-name.test 2_0001.dmn -i "Greeting Message" 2>&1 | ansifilter -T