option(ENABLE_ACTIONDB_PARSER "Enable actiondb-parser module" ON)

if (ENABLE_ACTIONDB_PARSER)
  add_lib_crate(actiondb-parser)
  add_cargo_bin_target(adbtool)

  install(FILES ${ACTIONDB_PARSER_LIB_TARGET} DESTINATION lib/syslog-ng)
  install(FILES ${ADBTOOL_PATH}
    PERMISSIONS OWNER_READ OWNER_WRITE OWNER_EXECUTE
                GROUP_READ GROUP_EXECUTE
                WORLD_READ WORLD_EXECUTE
    DESTINATION bin/)
endif()
