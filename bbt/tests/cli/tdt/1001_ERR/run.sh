#!/usr/bin/env bash

# test decision table
dmntk tdt -s non-existing-name.test input.dtb 2>&1 | ansifilter -T
