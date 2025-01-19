use std::fmt::{format, Debug};
use std::fs;
use satway_build::CompileInfo;

pub fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let compile_info:CompileInfo=CompileInfo::load_from_env();
    fs::write("src/context/info.txt",compile_info.save_to_str(true)).expect("Unable to write file");
}
