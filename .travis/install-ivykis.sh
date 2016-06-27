#!/bin/bash -xe

cd "$TRAVIS_BUILD_DIR"
curl -sL https://github.com/balabit/ivykis/archive/master.zip -o ivykis.zip
unzip ivykis.zip
cd ivykis-master
autoreconf -i
./configure --prefix="$HOME/install/syslog-ng"
make install

EXITCODE=$?

exit $EXITCODE
