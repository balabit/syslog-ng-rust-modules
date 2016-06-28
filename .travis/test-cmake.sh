#!/bin/bash -xe

mkdir build
cd build
cmake -DCMAKE_INSTALL_PREFIX=$HOME/install/syslog-ng \
  -DENABLE_REGEX_PARSER=ON \
  -DENABLE_PYTHON_PARSER=ON \
  -DENABLE_ACTIONDB_PARSER=ON \
  -DENABLE_CORRELATION_PARSER=ON ..
make install
cd ..
