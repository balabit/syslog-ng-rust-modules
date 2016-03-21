cmake_minimum_required(VERSION 2.8)
project(syslog-ng-rust-modules NONE)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake/Modules")

find_package(Rust)
find_package(Cargo)
