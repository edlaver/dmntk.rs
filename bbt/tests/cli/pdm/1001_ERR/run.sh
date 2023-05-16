#!/usr/bin/env bash

# parse DMN model with non existing model file name
dmntk pdm -c never non-existing-name.dmn 2>&1
