# Rust modules for syslog-ng

## Requirements

* `cmake` (at least 2.8)
* `rustc` (at least 1.5)
* `cargo`
* `syslog-ng` (at least 3.8)
* a C compiler

## Building

1. Clone this repository
1. Create build directory
```
mkdir build
```
1. Step into the build directory
```
cd build
```
1. Generate build system (the individual modules may need additional flags for CMake):
```
cmake -DCMAKE_INSTALL_PREFIX=/home/btibi/install/syslog-ng ..
```

 You can also use CMake's graphical configurator (`cmake-gui`). `CMAKE_INSTALL_PREFIX` should point to the directory, where syslog-ng was installed.
 The default build type is `Debug`, you can build a highly optimized version with the `-DCMAKE_BUILD_TYPE=Release` flag.
1. Build the project:
```
make
```
1. Install the project:
```
make install
```

### python-parser

Check [README](python-parser/README.md) for more detailed information.

#### Requirements

* `python-3.4` development library
* `ENABLE_PYTHON_PARSER` CMake flag should be set to ON

#### Additional CMake flags

* `PYTHON_LIBRARY`: if CMake cannot find the Python development library, specify the location manually:

 ```
cmake -DPYTHON_LIBRARY=/usr/lib/x86_64-linux-gnu/libpython3.4m.so -DCMAKE_INSTALL_PREFIX=/home/btibi/install/syslog-ng ..
```

### regex-parser

Check [README](regex-parser/README.md) for more detailed information.

#### Requirements

* `ENABLE_REGEX_PARSER` CMake flag should be set to ON

### actiondb-parser

Check [actiondb-parser](actiondb-parser/README.md) and
[actiondb](https://github.com/ihrwein/actiondb/blob/master/README.md) for more detailed information.

#### Requirements

* `ENABLE_ACTIONDB_PARSER` CMake flag should be set to ON
