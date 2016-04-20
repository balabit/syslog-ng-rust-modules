cmake_minimum_required(VERSION 2.8)
project(syslog-ng-rust-modules NONE)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake/Modules")

find_package(Rust REQUIRED)
find_package(Cargo REQUIRED)
find_package(PkgConfig REQUIRED)

pkg_check_modules (SYSLOG_NG REQUIRED syslog-ng)
pkg_check_modules (IVYKIS REQUIRED ivykis)

include(UseRust)
include(UsePkgConfig)

pkg_check_variable(syslog-ng moduledir)
pkg_check_variable(syslog-ng scldir)
message("ModuleDir: ${SYSLOG_NG_MODULEDIR}")
message("SclDir: ${SYSLOG_NG_SCLDIR}")

add_subdirectory(python-parser)
add_subdirectory(regex-parser)
add_subdirectory(actiondb-parser)
add_subdirectory(correlation-parser)
