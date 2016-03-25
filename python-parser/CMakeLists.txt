find_package(PythonLibs 3.4)

if (PYTHONLIBS_FOUND)
    option(ENABLE_PYTHON_PARSER "Enable python-parser module" ON)
else()
    option(ENABLE_PYTHON_PARSER "Enable python-parser module" OFF)
endif()

if (ENABLE_PYTHON_PARSER)
  add_cargo_lib_target(python-parser)

  install(FILES ${PYTHON_PARSER_PATH} DESTINATION ${SYSLOG_NG_MODULEDIR})
  install(DIRECTORY scl/ DESTINATION ${SYSLOG_NG_SCLDIR})
endif()
