cmake_minimum_required(VERSION 2.8)
project(syslog-ng-rust-modules NONE)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake/Modules")

find_package(Rust REQUIRED)
find_package(Cargo REQUIRED)
find_package(PkgConfig REQUIRED)

pkg_check_modules(syslog-ng REQUIRED QUIET)

include(UseRust)

add_subdirectory(dummy-lib)
