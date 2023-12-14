fn main() {
    let mut args = std::env::args();
    let _ = args.next().unwrap();
    let bin_file = args.next().unwrap();
    println!("Binary File {bin_file}");

    let mut vm = icicle_vm::build(&icicle_vm::cpu::Config {
        triple: "x86_64-linux-gnu".parse().unwrap(),
        enable_jit: false,
        enable_jit_mem: false,
        enable_recompilation: false,
        track_uninitialized: false,
        enable_shadow_stack: false,
        optimize_instructions: false,
        optimize_block: false,
        ..icicle_vm::cpu::Config::default()
    })
    .unwrap();
    vm.env = icicle_vm::env::build_auto(&mut vm).unwrap();
    vm.env.load(&mut vm.cpu, bin_file.as_bytes()).unwrap();

    // let it run, so it decompile the basic
    let end = vm.run();
    dbg!(end);

    for block_group in vm.code.map.values() {
        // print the disassembly
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
                println!("0x{addr:x}  {disasm}");
            }
        }

        //print the lifted code
        let lifted = block_group
            .to_string(&vm.code.blocks, &vm.cpu.arch.sleigh, true)
            .unwrap();
        println!("{lifted}");
    }
}
