extern crate pkg_config;

fn main() {
    let res = pkg_config::find_library("syslog-ng");
    match res {
        Ok(value) => {
            for dir in value.link_paths {
                println!("cargo:rustc-link-search=native={:?}", dir);
            }
            println!("cargo:rustc-link-lib=dylib=syslog-ng")
        },
        Err(err) => {
            println!("libsyslog-ng.so is not found by pkg-config: {}", err);
        }
    }
}
