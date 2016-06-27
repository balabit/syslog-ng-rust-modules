#!/bin/bash -x

function is_nightly_feature_used {
	grep -q nightly Cargo.toml;
}

cd $(dirname "$1")

if ! is_nightly_feature_used; then
	export TRAVIS_CARGO_NIGHTLY_FEATURE=""
fi

travis-cargo build &&
  travis-cargo test &&
  travis-cargo bench &&
  travis-cargo --only stable doc
