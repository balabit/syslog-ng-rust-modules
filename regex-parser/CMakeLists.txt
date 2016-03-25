option(ENABLE_REGEX_PARSER "Enable regex-parser module" ON)

if (ENABLE_REGEX_PARSER)
  add_cargo_lib_target(regex-parser)

  install(FILES ${REGEX_PARSER_LIB_TARGET} DESTINATION lib/syslog-ng)
endif()
