#!/usr/bin/env bash

rm -rf output.html

dmntk xdm 2_0001.dmn output.html

head -n 10 output.html

rm -rf output.html