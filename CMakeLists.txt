cmake_minimum_required(VERSION 2.8)
project(syslog-ng-rust-modules NONE)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake/Modules")

find_package(Rust REQUIRED)
find_package(Cargo REQUIRED)
find_package(PkgConfig REQUIRED)

pkg_check_modules (SYSLOG_NG REQUIRED syslog-ng)

include(UseRust)

add_subdirectory(python-parser)
add_subdirectory(regex-parser)
add_subdirectory(actiondb-parser)
