use std::env::vars_os;
use std::fmt::{format, Debug};
use std::{env, fs};
use std::path::Path;
use satway_build::CompileInfo;

pub fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let compile_info:CompileInfo=CompileInfo::load_from_env();

    fs::write("./target/compile_info.txt",compile_info.save_to_str(true)).expect("Unable to write file");
}
