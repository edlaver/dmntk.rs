#!/usr/bin/env bash

rm -rf output.html

touch output.html

chmod -w output.html

dmntk xdt input.dtb output.html 2>&1

rm -rf output.html