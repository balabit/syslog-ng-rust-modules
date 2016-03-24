# Rust modules for syslog-ng

## Requirements

* `cmake` (at least 2.8)
* `rustc`
* `cargo`
* `syslog-ng` (at least 3.8)

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

 `CMAKE_INSTALL_PREFIX` should point to the directory, where syslog-ng was installed.
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

#### Additional CMake flags

* `PYTHON_LIBRARY`: if CMake cannot find the Python development library, specify the location manually:

 ```
cmake -DPYTHON_LIBRARY=/usr/lib/x86_64-linux-gnu/libpython3.4m.so -DCMAKE_INSTALL_PREFIX=/home/btibi/install/syslog-ng ..
```
