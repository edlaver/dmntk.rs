#!/usr/bin/env bash

# test feel expression
dmntk tfe input.test non-existing-name.feel 2>&1 | ansifilter -T
