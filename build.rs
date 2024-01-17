fn main() {
    let cpp_src_path = "src";
    let cpp_include_path = "src"; 
    let ghidra_include_path = "ghidra/Ghidra/Features/Decompiler/src/decompile/cpp";

    // Compile and link the C++ code
    cc::Build::new()
        .cpp(true)
        .file(format!("{}/raw_pcode_generator.cc", cpp_src_path))
        .include(cpp_include_path) 
        .include(ghidra_include_path)
        .flag("-std=c++11")
        .flag("-Wno-unused-parameter")
        .compile("raw_pcode_generator");

    // Link the precompiled static library libdecomp.a
    let lib_dir = "/home/kgorna/Documents/tools/pcode-generator/ghidra/Ghidra/Features/Decompiler/src/decompile/cpp";
    let lib_name = "decomp";  // without the 'lib' prefix and '.a' suffix

    println!("cargo:rustc-link-search=native={}", lib_dir);
    println!("cargo:rustc-link-lib=static={}", lib_name);
    println!("cargo:rerun-if-changed={}/raw_pcode_generator.hh", cpp_src_path);
    println!("cargo:rerun-if-changed={}/raw_pcode_generator.cc", cpp_src_path);
    println!("cargo:rerun-if-changed=src/main.rs");
}
