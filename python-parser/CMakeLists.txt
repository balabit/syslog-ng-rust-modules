find_package(PythonLibs 3.4)

if (PYTHONLIBS_FOUND)
    option(ENABLE_PYTHON_PARSER "Enable python-parser module" ON)
else()
    option(ENABLE_PYTHON_PARSER "Enable python-parser module" OFF)
endif()

if (ENABLE_PYTHON_PARSER)
  add_lib_crate(python-parser)

  install(FILES ${PYTHON_PARSER_LIB_TARGET} DESTINATION lib/syslog-ng)
endif()
