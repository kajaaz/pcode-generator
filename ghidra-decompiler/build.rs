fn main() {
    let ghidra_include_path = "../ghidra/Ghidra/Features/Decompiler/src/decompile/cpp";
    let ghidra_files = DECOMPILER_FILES
        .iter()
        .map(|file| format!("{ghidra_include_path}/{file}"));

    // Compile and link the C++ code
    cxx_build::bridge("src/lib.rs")
        .flag("-std=c++14")
        .flag("-Wno-unused-parameter")
        .warnings(false)
        .files(ghidra_files)
        .file("src/wrapper.cc")
        .include(ghidra_include_path)
        .include("src")
        .compile("cxxbridge");

    println!("cargo:rerun-if-changed=src/wrapper.cc",);
    println!("cargo:rerun-if-changed=src/wrapper.hh",);
    println!("cargo:rerun-if-changed={ghidra_include_path}");
}

// TODO not all files are required to pcode dump, remove uncessary files to
// speed up compilation
const DECOMPILER_FILES: &[&str] = &[
    "action.cc",
    "address.cc",
    "analyzesigs.cc",
    "architecture.cc",
    "bfd_arch.cc",
    "blockaction.cc",
    "block.cc",
    "callgraph.cc",
    "capability.cc",
    "cast.cc",
    "codedata.cc",
    "comment.cc",
    "condexe.cc",
    "consolemain.cc",
    "context.cc",
    "coreaction.cc",
    "cover.cc",
    "cpool.cc",
    "crc32.cc",
    "database.cc",
    "double.cc",
    "dynamic.cc",
    "emulate.cc",
    "emulateutil.cc",
    "filemanage.cc",
    "float.cc",
    "flow.cc",
    "fspec.cc",
    "funcdata_block.cc",
    "funcdata.cc",
    "funcdata_op.cc",
    "funcdata_varnode.cc",
    "globalcontext.cc",
    "grammar.cc",
    "graph.cc",
    "heritage.cc",
    "ifacedecomp.cc",
    "ifaceterm.cc",
    "inject_sleigh.cc",
    "interface.cc",
    "jumptable.cc",
    "libdecomp.cc",
    "loadimage_bfd.cc",
    "loadimage.cc",
    "loadimage_xml.cc",
    "marshal.cc",
    "memstate.cc",
    "merge.cc",
    "modelrules.cc",
    "opbehavior.cc",
    "op.cc",
    "opcodes.cc",
    "options.cc",
    "override.cc",
    "paramid.cc",
    "pcodecompile.cc",
    "pcodeinject.cc",
    "pcodeparse.cc",
    "pcoderaw.cc",
    "prefersplit.cc",
    "prettyprint.cc",
    "printc.cc",
    "printjava.cc",
    "printlanguage.cc",
    "rangeutil.cc",
    "raw_arch.cc",
    "ruleaction.cc",
    "rulecompile.cc",
    "semantics.cc",
    "signature.cc",
    "sleigh_arch.cc",
    "sleighbase.cc",
    "sleigh.cc",
    "slghpatexpress.cc",
    "slghpattern.cc",
    "slghsymbol.cc",
    "space.cc",
    "stringmanage.cc",
    "subflow.cc",
    "testfunction.cc",
    "transform.cc",
    "translate.cc",
    "type.cc",
    "typeop.cc",
    "unify.cc",
    "unionresolve.cc",
    "userop.cc",
    "variable.cc",
    "varmap.cc",
    "varnode.cc",
    "xml_arch.cc",
    "xml.cc",
];
