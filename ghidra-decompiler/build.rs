use std::path::Path;

fn main() {
    let num_jobs = std::env::var("NUM_JOBS").unwrap();
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let disassembler_dir =
        format!("{manifest_dir}/../ghidra/Ghidra/Features/Decompiler/src/decompile/cpp");

    let (static_lib, static_lib_name) = match std::env::var("PROFILE").unwrap().as_str() {
        "debug" => ("libdecomp_dbg.a", "decomp_dbg"),
        "release" | _ => ("libdecomp.a", "decomp"),
    };

    // rerun if any file in ghidra change
    for file in std::fs::read_dir(&disassembler_dir).unwrap() {
        let file = file.unwrap();
        let path = file.path();
        // rerun if any of those files change
        match file.path().extension().map(|ext| ext.to_str()).flatten() {
            // code file
            Some("cc") | Some("hh") | Some("c") | Some("h") | Some("y") | Some("l") => {
                println!("cargo:rerun-if-changed={}", path.to_str().unwrap())
            }
            // Makefile
            _ if file.file_name().to_str().unwrap() == "Makefile" => {
                println!("cargo:rerun-if-changed={}", path.to_str().unwrap())
            }
            _ => {}
        }
    }

    println!("cargo:rerun-if-changed=src/wrapper.cc");
    println!("cargo:rerun-if-changed=src/wrapper.hh");
    println!("cargo:rustc-link-search={disassembler_dir}");
    println!("cargo:rustc-link-lib=static={static_lib_name}");

    // TODO clean the dir first? I think make can't cache the changes by default

    // compile the ghidra decompiler into a static lib
    assert!(std::process::Command::new("make")
        .args(["-j", &num_jobs, "-C", &disassembler_dir, static_lib])
        .status()
        .unwrap()
        .success());

    // generate the bindings file
    let bindings = bindgen::Builder::default()
        .clang_arg(&format!("-I{disassembler_dir}"))
        .clang_arg("-std=c++14")
        //.disable_untagged_union()
        .enable_cxx_namespaces()
        .vtable_generation(true)
        .allowlist_type("ghidra::.*")
        .blocklist_item("pointer")
        .blocklist_item("const_pointer")
        .opaque_type("__gnu_cxx::.*")
        .opaque_type("std::.*")
        .header("src/lib.hpp")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = Path::new(&out_dir);
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Compile and link the C++ code
    cxx_build::bridge("src/lib.rs")
        .flag("-std=c++14")
        .flag("-Wno-unused-parameter")
        .warnings(false)
        .include("src")
        .include(disassembler_dir)
        .file("src/wrapper.cc")
        .compile("cxxbridge");
}
