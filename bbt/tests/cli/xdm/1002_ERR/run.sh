#!/usr/bin/env bash

# remove existing output file
rm -rf output.html

# create empty output file
touch output.html

# make this file read-only
chmod -w output.html

# export DMN model to HTML
dmntk xdm 2_0001.dmn output.html 2>&1

# remove output file
rm -rf output.html
