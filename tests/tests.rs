extern crate compiletest_rs as compiletest;

use std::path::PathBuf;
use std::env;

fn run_mode(mode: &'static str) {
    let mut config = compiletest::default_config();
    let cfg_mode = mode.parse().ok().expect("Invalid mode");

    config.mode = cfg_mode;
    config.src_base = PathBuf::from(format!("tests/{}", mode));

    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let search_path_strings: Vec<_> = ["LD_LIBRARY_PATH", "DYLD_LIBRARY_PATH"]
                                          .iter()
                                          .filter_map(env::var_os)
                                          .collect();
    let search_paths = search_path_strings.iter()
                                          .flat_map(env::split_paths)
                                          .filter_map(|path| {
                                              if path.starts_with(&manifest_dir) {
                                                  Some(format!("-L {}", path.display()))
                                              } else {
                                                  None
                                              }
                                          })
                                          .collect::<Vec<String>>()
                                          .join(" ");

    config.target_rustcflags = Some(search_paths);

    compiletest::run_tests(&config);
}

#[test]
fn compile_test() {
    run_mode("compile-fail");
}
