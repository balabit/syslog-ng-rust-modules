option(ENABLE_CORRELATION_PARSER "Enable correlation-parser module" ON)

if (ENABLE_CORRELATION_PARSER)
  add_cargo_lib_target(correlation-parser)

  install(FILES ${CORRELATION_PARSER_PATH} DESTINATION lib/syslog-ng)
  install(DIRECTORY scl/ DESTINATION share/syslog-ng/include/scl)
endif()
