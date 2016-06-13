# Exports:
# RUSTUP_EXECUTABLE: path to the multirust binary
# RUSTUP_TOOLCHAIN_BIN_DIR: path to the directory, where rustc, cargo, etc. installed

find_program(RUSTUP rustup PATHS PATH_SUFFIXES bin)

if (RUSTUP_EXECUTABLE)
    set (COMMAND ${RUSTUP_EXECUTABLE} which rustc)
    execute_process(COMMAND ${COMMAND} OUTPUT_VARIABLE WHICH_RUSTC OUTPUT_STRIP_TRAILING_WHITESPACE)
    get_filename_component(RUSTUP_TOOLCHAIN_BIN_DIR ${WHICH_RUSTC} DIRECTORY CACHE)
    set(RUSTUP_FOUND TRUE CACHE INTERNAL "Rustup found")
else (RUSTUP_EXECUTABLE)
    set(RUSTUP_FOUND FALSE CACHE INTERNAL "Rustup not found")
endif(RUSTUP_EXECUTABLE)
