option(ENABLE_ACTIONDB_PARSER "Enable actiondb-parser module" ON)

if (ENABLE_ACTIONDB_PARSER)
  add_cargo_lib_target(actiondb-parser)
  add_cargo_bin_target(adbtool)

  install(FILES ${ACTIONDB_PARSER_PATH} DESTINATION ${SYSLOG_NG_MODULEDIR})
  install(FILES ${ADBTOOL_PATH}
    PERMISSIONS OWNER_READ OWNER_WRITE OWNER_EXECUTE
                GROUP_READ GROUP_EXECUTE
                WORLD_READ WORLD_EXECUTE
    DESTINATION bin/)
  install(DIRECTORY scl/ DESTINATION ${SYSLOG_NG_SCLDIR})
endif()
