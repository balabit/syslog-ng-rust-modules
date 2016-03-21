# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

# The module defines the following variables:
#   RUST_FOUND - true if the Rust was found
#   RUST_EXECUTABLE - path to the executable
#   RUST_VERSION - Rust version number
#   RUST_NIGHTLY - TRUE if the compiler is a nightly version, FALSE otherwise
# Example usage:
#   find_package(Rust 0.12.0 REQUIRED)

find_program(MULTIRUST_EXECUTABLE multirust PATHS PATH_SUFFIXES bin)

if (MULTIRUST_EXECUTABLE)
  set (COMMAND ${MULTIRUST_EXECUTABLE} which rustc)
  execute_process(COMMAND ${COMMAND} OUTPUT_VARIABLE WHICH_RUSTC OUTPUT_STRIP_TRAILING_WHITESPACE)
  get_filename_component(MULTIRUST_TOOLCHAIN_BIN_DIR ${WHICH_RUSTC} DIRECTORY CACHE)
else ()
  set(MULTIRUST_TOOLCHAIN_BIN_DIR "" CACHE FILEPATH)
endif()

find_program(RUST_EXECUTABLE rustc HINTS ${MULTIRUST_TOOLCHAIN_BIN_DIR} PATHS PATH_SUFFIXES bin)

if (RUST_EXECUTABLE)
    set(COMMAND ${RUST_EXECUTABLE} --version)
    execute_process(COMMAND ${COMMAND} OUTPUT_VARIABLE RUST_VERSION_OUTPUT OUTPUT_STRIP_TRAILING_WHITESPACE)
    if(RUST_VERSION_OUTPUT MATCHES "rustc ([0-9]+\\.[0-9]+\\.[0-9]+)(-nightly)?")
      set(RUST_VERSION ${CMAKE_MATCH_1} CACHE INTERNAL "doc")
      if(CMAKE_MATCH_2)
        set(RUST_NIGHTLY TRUE CACHE BOOL "Nightly compiler")
      else()
        set(RUST_NIGHTLY FALSE CACHE BOOL "Nightly compiler")
      endif()
    endif()
endif()
mark_as_advanced(RUST_EXECUTABLE)

include(FindPackageHandleStandardArgs)
find_package_handle_standard_args(Rust REQUIRED_VARS RUST_EXECUTABLE RUST_VERSION VERSION_VAR RUST_VERSION)
