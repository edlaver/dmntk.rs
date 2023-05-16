#!/usr/bin/env bash

# test decision table
dmntk tdt -s input.test input.dtb 2>&1 | ansifilter -T
