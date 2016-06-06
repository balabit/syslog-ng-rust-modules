#!/bin/bash

OLDPWD=`pwd`
cd $(dirname $1)
cargo test
EXITCODE=$?
cd $OLDPWD

exit $EXITCODE
