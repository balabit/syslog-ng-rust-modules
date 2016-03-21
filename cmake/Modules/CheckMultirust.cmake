# Exports:
# MULTIRUST_EXECUTABLE: path to the multirust binary
# MULTIRUST_TOOLCHAIN_BIN_DIR: path to the directory, where rustc, cargo, etc. installed

find_program(MULTIRUST_EXECUTABLE multirust PATHS PATH_SUFFIXES bin)

if (MULTIRUST_EXECUTABLE)
  set (COMMAND ${MULTIRUST_EXECUTABLE} which rustc)
  execute_process(COMMAND ${COMMAND} OUTPUT_VARIABLE WHICH_RUSTC OUTPUT_STRIP_TRAILING_WHITESPACE)
  get_filename_component(MULTIRUST_TOOLCHAIN_BIN_DIR ${WHICH_RUSTC} DIRECTORY CACHE)
else ()
  set(MULTIRUST_TOOLCHAIN_BIN_DIR "" CACHE FILEPATH)
endif()
