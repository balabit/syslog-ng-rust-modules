#!/bin/bash -xe

cd "$TRAVIS_BUILD_DIR"
git clone https://github.com/balabit/syslog-ng.git && cd syslog-ng
mkdir build && cd build
cmake -DCMAKE_C_FLAGS="$CMAKE_C_FLAGS" \
  -DENABLE_JAVA:BOOL=OFF \
  -DENABLE_GRADLE:BOOL=OFF \
  -DENABLE_PYTHON:BOOL=OFF \
  -DCMAKE_INSTALL_PREFIX="$HOME/install/syslog-ng" ..
make -j install
EXITCODE=$?

exit $EXITCODE
