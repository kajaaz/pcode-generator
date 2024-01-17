use std::fs::{self, File};
use std::io::Write;
use std::env;
use std::path::PathBuf;
use icicle_vm;
use pcode;

pub fn generate_high_pcode(filename: &str) {
    let mut args = env::args();
    let _ = args.next().unwrap(); // Skip the first argument which is the program name
    let bin_file = match args.next() {
        Some(file) => file,
        None => {
            eprintln!("Usage: cargo run <path_to_binary_file>");
            return;
        }
    };
    println!("Binary File: {bin_file}");

    println!("Building VM...");
    let mut vm = match icicle_vm::build(&icicle_vm::cpu::Config {
        triple: "x86_64-linux-gnu".parse().unwrap(),
        enable_jit: false,
        enable_jit_mem: false,
        enable_recompilation: false,
        track_uninitialized: false,
        enable_shadow_stack: false,
        optimize_instructions: false,
        optimize_block: false,
        ..icicle_vm::cpu::Config::default()
    }) {
        Ok(vm) => vm,
        Err(e) => {
            eprintln!("Failed to build VM: {:?}", e);
            return;
        }
    };

    println!("Setting up VM environment...");
    vm.env = match icicle_vm::env::build_auto(&mut vm) {
        Ok(env) => env,
        Err(e) => {
            eprintln!("Failed to set up VM environment: {:?}", e);
            return;
        }
    };

    println!("Loading binary file into VM...");
    if let Err(e) = vm.env.load(&mut vm.cpu, bin_file.as_bytes()) {
        eprintln!("Failed to load binary file: {:?}", e);
        return;
    }

    println!("Running VM...");
    let end = vm.run();
    dbg!(end);

    // New : Find the current executable's directory
    let exe_path = env::current_exe().expect("Failed to get the current executable path");
    let exe_dir = exe_path.parent().expect("Failed to get the executable directory");

    // New : Navigate up two levels from the executable's directory to reach the project root
    let project_root = exe_dir.parent().and_then(|p| p.parent()).expect("Failed to find the project root directory");

    // New : Create the "results" directory in the project root
    let results_dir = project_root.join("results");
    if let Err(e) = fs::create_dir_all(&results_dir) {
        eprintln!("Failed to create results directory: {:?}", e);
        return;
    }

    // New : Extract the filename from the provided file path
    let file_stem = PathBuf::from(filename)
        .file_stem()
        .expect("Failed to extract file stem")
        .to_str()
        .expect("Failed to convert file stem to string")
        .to_owned();

    // New : Modify the output filename to be inside the "results" directory
    let output_filename = results_dir.join(format!("{}_high_pcode.txt", file_stem));
    let mut output_file = match File::create(&output_filename) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create output file at {:?}: {:?}", output_filename, e);
            return;
        }
    };
    
    // New : Print the full path of the output file for verification
    println!("Output file will be created at: {:?}", output_filename);

    println!("Starting p-code generation and file writing...");
    for block_group in vm.code.map.values() {
        for block in &vm.code.blocks[block_group.range()] {
            for stmt in &block.pcode.instructions {
                if !matches!(stmt.op, pcode::Op::InstructionMarker) {
                    continue;
                }
                let addr = stmt.inputs.first().as_u64();
                let _len = stmt.inputs.second().as_u64();
                let disasm = vm
                    .code
                    .disasm
                    .get(&addr)
                    .map_or("invalid_instruction", |x| x.as_str());

                if let Err(e) = writeln!(output_file, "0x{addr:x}  {disasm}") {
                    eprintln!("Failed to write to output file: {:?}", e);
                    return;
                }
            }
        }

        let lifted = match block_group
            .to_string(&vm.code.blocks, &vm.cpu.arch.sleigh, true) {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Failed to convert block group to string: {:?}", e);
                return;
            }
        };

        if let Err(e) = writeln!(output_file, "{lifted}") {
            eprintln!("Failed to write to output file: {:?}", e);
            return;
        }
    }
}
