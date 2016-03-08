#!/bin/bash

set -e

MODULE_DIR=$(pkg-config --variable=moduledir syslog-ng)

if test -z "$MODULE_DIR" ; then
    echo "Don't know where to install the modules"
    exit 1
fi


for i in "$@"; do
    case $i in
        --debug)
        BUILD="debug"
        shift # past argument=value
        ;;
        --release)
        BUILD="release"
        shift # past argument=value
        ;;
        *)
        # unknown option
        ;;
    esac
done

if test -z $BUILD; then
    BUILD="debug"
fi

echo "BUILD           = ${BUILD}"
echo "MODULE_DIR      = ${MODULE_DIR}"

for i in **/target/$BUILD/*.so; do
    echo "Copying '$i' to '$MODULE_DIR'"
    cp "$i" "$MODULE_DIR"
done
