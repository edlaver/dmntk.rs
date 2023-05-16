#!/usr/bin/env bash

# remove existing output file
rm -rf output.html

# create empty output file
touch output.html

# make this file read-only
chmod -w output.html

# export decision table to HTML
dmntk xdt input.dtb output.html 2>&1

# remove output file
rm -rf output.html
