#!/usr/bin/env bash

###############################################################################
# Dependencies:
#
# $ sudo dnf install lcov
# $ rustup component add llvm-tools-preview
# $ cargo install grcov
#
###############################################################################

WORKING_DIRECTORY=$(pwd)
DMNTK_BINARY_PATH="$WORKING_DIRECTORY"/target/debug
MANUAL_TESTS_DIRECTORY="$WORKING_DIRECTORY"/../dmntk.manual.tests
TEST_RUNNER_DIRECTORY="$WORKING_DIRECTORY"/../dmntk.test.runner

# clean before proceeding
cargo clean

# set instrumenting variables
export CARGO_INCREMENTAL=0
export RUSTDOCFLAGS="-Cpanic=abort"
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
# set DMNTK features
export CARGO_FEATURE_PARSING_TABLES=1

if [ -n "$1" ]; then
  # run tests only for specified package
  cargo +nightly test -p "$1"
else
  # run all tests including including manual tests
  cargo +nightly test
  # build the whole binary before running tests
  cargo +nightly build
  # run manual tests to collect the coverage of the code executed from command-line
  echo "$MANUAL_TESTS_DIRECTORY"
  if [[ -d "$MANUAL_TESTS_DIRECTORY" ]]
  then
    export PATH=$DMNTK_BINARY_PATH:$PATH
    cd "$MANUAL_TESTS_DIRECTORY" || exit 1
    ./run.sh
    cd "$WORKING_DIRECTORY" || exit 1
  fi
  # collect the coverage from TCK tests
  echo "$TEST_RUNNER_DIRECTORY"
  if [[ -d "$TEST_RUNNER_DIRECTORY" ]]
  then
    export PATH=$DMNTK_BINARY_PATH:$PATH
    dmntk srv > /dev/null 2>&1 &
    _pid=$!
    sleep 0.1
    cd "$TEST_RUNNER_DIRECTORY" || exit 1
    dmntk-test-runner config-all.yml
    cd "$WORKING_DIRECTORY" || exit 1
    kill -s SIGINT "$_pid"
    sleep 0.1
  fi
fi

# prepare output directories for coverage results
mkdir ./target/lcov
mkdir ./target/coverage
# generate coverage info
grcov . --llvm -s . -t lcov --branch --ignore-not-existing --ignore "*cargo*" --ignore "*chrono-tz*" --ignore "*tests*" -o ./target/lcov/lcov.info
# generate coverage report
genhtml -t "Decision Model and Notation Toolkit" -q -o ./target/coverage ./target/lcov/lcov.info
# display final message
echo ""
echo "Open coverage report: file://$WORKING_DIRECTORY/target/coverage/index.html"
echo ""

# reformat generated code
cargo +nightly fmt -p dmntk-feel-parser