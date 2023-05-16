#!/usr/bin/env bash

# remove existing output file
rm -rf output.html

# export decision table to HTML
dmntk xdt input.dtb output.html 2>&1

# send the generated HTML content to standard output
head -n 10 output.html

# remove output file
rm -rf output.html
