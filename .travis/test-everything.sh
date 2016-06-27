#!/bin/bash -xe

find . -name Cargo.toml |
	xargs -n 1 .travis/cargo-test.sh
