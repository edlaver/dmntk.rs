#!/usr/bin/env bash

# test decision table
dmntk tdt -s input.test non-existing-name.dtb 2>&1 | ansifilter -T
