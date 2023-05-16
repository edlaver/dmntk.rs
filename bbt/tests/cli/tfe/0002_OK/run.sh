#!/usr/bin/env bash

# test feel expression
dmntk tfe input.test input.feel 2>&1 | ansifilter -T
