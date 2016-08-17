// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

extern crate pkg_config;
extern crate gcc;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

const RUST_DEPS_A_NAME: &'static str = "syslog-ng-native-connector";

fn create_plugins(parser_name: Option<&str>) -> String {
    let head = r#"
static Plugin native_plugins[] =
{
    "#;
    let tail = r#"
};
    "#;
    let middle = match parser_name {
        Some(name) => {
            format!(r#"
  {{
    .type = LL_CONTEXT_PARSER,
    .name = "{name}",
    .parser = &native_parser,
 }},
            "#,
                    name = name)
        }
        None => format!(""),
    };

    format!("{}{}{}", head, middle, tail)
}

fn create_module_init(canonical_name: &str) -> String {
    let name = canonical_name.replace("-", "_");
    format!(r#"
gboolean
{name}_module_init(GlobalConfig *cfg, CfgArgs *args)
{{
  plugin_register(cfg, native_plugins, G_N_ELEMENTS(native_plugins));
  return TRUE;
}}
    "#,
            name = name)
}

fn create_module_info(canonical_name: &str, description: &str) -> String {
    format!(r#"
const ModuleInfo module_info =
{{
  .canonical_name = "{name}",
  .version = SYSLOG_NG_VERSION,
  .description = "{description}",
  .core_revision = VERSION_CURRENT_VER_ONLY,
  .plugins = native_plugins,
  .plugins_len = G_N_ELEMENTS(native_plugins),
}};
    "#,
            name = canonical_name,
            description = description)
}

fn get_out_dir_file_path(filename: &str) -> PathBuf {
    let out_dir = env::var("OUT_DIR").unwrap();
    Path::new(&out_dir).join(filename)
}

pub fn create_module_content(canonical_name: &str,
                             description: &str,
                             parser_name: Option<&str>)
                             -> String {
    let header = r#"
#include "cfg-parser.h"
#include "plugin.h"
#include "plugin-types.h"

CfgParser native_parser;
    "#;

    format!("{header}{plugins}{module_init}{module_info}",
            header = header,
            plugins = create_plugins(parser_name),
            module_init = create_module_init(canonical_name),
            module_info = create_module_info(canonical_name, description))
}

fn link_against_rust_deps() {
    match pkg_config::Config::new().statik(true).find(RUST_DEPS_A_NAME) {
        Ok(_) => {}
        Err(err) => {
            println!("{}", err);
        }
    }
}

fn compile_and_link_module<P: AsRef<Path>>(dest_path: P) {
    let mut compiler = gcc::Config::new();
    match pkg_config::find_library("syslog-ng") {
        Ok(lib) => {
            for i in lib.include_paths {
                compiler.include(i);
            }

            compiler.flag("-c")
                    .flag("-std=gnu99")
                    .file(dest_path)
                    .compile("librust-module.a");
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}

fn write_module_content_to_file<P: AsRef<Path>>(content: &str, path: P) {
    let mut module_file = File::create(&path).unwrap();
    module_file.write((&content).as_bytes())
               .ok()
               .expect("Failed to write module info during build");
}

fn link_against_module(content: &str) {
    let module_file_name = "module.c";
    let dest_path = get_out_dir_file_path(module_file_name);
    write_module_content_to_file(&content, &dest_path);
    compile_and_link_module(&dest_path);
}

/// Generates a module for syslog-ng.
///
/// This method generates code which describes a syslog-ng module. A module has a `canonical_name`
/// (like `"foo"`), a `description` (like "This module contains plugins to ....") and an optional
/// parser plugin. `parser_name` represents the name of the parser, if it's `None` the module
/// doesn't contain any plugin.
///
/// This method must be called in a build script (`build.rs`).
pub fn create_module(canonical_name: &str, description: &str, parser_name: Option<&str>) {
    link_against_rust_deps();
    let module_content = create_module_content(canonical_name, description, parser_name);
    link_against_module(&module_content);
}
