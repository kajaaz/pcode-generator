use icicle_vm;
use pcode;
use std::env;
use std::io::Write;

use crate::pcode_generator;

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

    let mut output_file = pcode_generator::create_output_file(filename, "high")
        .expect("Unable to create the output file");

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

        let lifted = match block_group.to_string(&vm.code.blocks, &vm.cpu.arch.sleigh, true) {
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
