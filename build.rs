use std::{env, fs, path::PathBuf};

use verilator::{
    gen::{Standard, Verilator},
    module::ModuleGenerator,
};

fn main() {
    let verilog_version = Standard::Verilog2005;
    let rtl_files = vec!["rtl/top.v"];

    // This envvar is set by cargo
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = PathBuf::from(out_dir);
    let _ = fs::remove_dir_all(&out_dir);
    fs::create_dir_all(&out_dir).expect("Couldn't create dir");

    // Generate CPP shim from RUST
    let mut module = ModuleGenerator::default();
    module.generate("src/main.rs");

    // Generate CPP from Verilog
    let mut verilator = Verilator::default();
    verilator.with_trace(true);

    for rtl in rtl_files {
        println!("cargo:rerun-if-changed={rtl}");

        verilator.file_with_standard(rtl, verilog_version);
    }

    verilator
        .file(out_dir.join("top.cpp"))
        .build("top");
}
