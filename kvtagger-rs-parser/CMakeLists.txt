option(ENABLE_KVTAGGER_RS_PARSER "Enable kvtagger-rs-parser module" ON)

if (ENABLE_KVTAGGER_RS_PARSER)
  add_cargo_lib_target(kvtagger-rs-parser)

  install(FILES ${KVTAGGER_RS_PARSER_PATH} DESTINATION lib/syslog-ng)
  install(DIRECTORY scl/ DESTINATION share/syslog-ng/include/scl)
endif()
