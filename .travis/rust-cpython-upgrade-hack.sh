#!/bin/bash -xe

# copied from https://github.com/dgrunwald/rust-cpython/blob/master/.travis.yml (7c59d554ec6470963b4880345f39b5a543c00496)

python -c "import sysconfig; print('\n'.join(map(repr,sorted(sysconfig.get_config_vars().items()))))"
mkdir ~/rust-installer
curl -sL https://static.rust-lang.org/rustup.sh -o ~/rust-installer/rustup.sh
sh ~/rust-installer/rustup.sh --prefix=~/rust --channel=$RUST_CHANNEL -y --disable-sudo
export PATH="$HOME/rust/bin:$PATH"
export PYTHON_LIB=$(python -c "import sysconfig; print(sysconfig.get_config_var('LIBDIR'))")
export LIBRARY_PATH="$LIBRARY_PATH:$PYTHON_LIB"
export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$PYTHON_LIB:$HOME/rust/lib"
rustc -V
