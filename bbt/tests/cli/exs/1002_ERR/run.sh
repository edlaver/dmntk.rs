#!/usr/bin/env bash

# delete existing examples
rm -rf examples

# create all directories and a file...
mkdir examples
mkdir examples/e1
touch examples/e1/e1.ctx

# ...and make this file read-only
chmod -w examples/e1/e1.ctx

# save examples
dmntk exs 2>&1

# delete examples
rm -rf examples
