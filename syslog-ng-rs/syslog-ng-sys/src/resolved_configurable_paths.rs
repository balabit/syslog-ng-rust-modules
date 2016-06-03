use c_char;

#[repr(C)]
pub struct ResolvedConfigurablePaths {
  cfgfilename : *const c_char,
  persist_file : *const c_char,
  ctlfilename : *const c_char,
  initial_module_path : *const c_char,
}

#[link(name = "syslog-ng")]
extern "C" {
    pub fn resolved_configurable_paths_init(slf: *mut ResolvedConfigurablePaths);
    pub static mut resolvedConfigurablePaths: ResolvedConfigurablePaths;
}
