#!/usr/bin/env bash

# delete existing examples
rm -rf examples

# create examples directory...
mkdir examples

# ...and make it read only
chmod -w examples

# try to save examples
dmntk exs 2>&1

# delete examples
rm -rf examples
