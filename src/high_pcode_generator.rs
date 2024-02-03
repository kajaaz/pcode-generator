use icicle_vm::{self, cpu::BlockGroup, Vm};

struct HighPcodeGenerator {
    vm: Vm,
    values: std::vec::IntoIter<BlockGroup>,
}

impl Iterator for HighPcodeGenerator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.values.next().map(|block_group| {
            block_group
                .to_string(&self.vm.code.blocks, &self.vm.cpu.arch.sleigh, true)
                .unwrap()
        })
    }
}

pub fn generate_high_pcode(
    bin_file: &str,
) -> Result<impl Iterator<Item = String>, Box<dyn std::error::Error>> {
    println!("Binary File: {bin_file}");

    println!("Building VM...");
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
    })?;

    println!("Setting up VM environment...");
    vm.env = icicle_vm::env::build_auto(&mut vm)?;

    println!("Loading binary file into VM...");
    vm.env.load(&mut vm.cpu, bin_file.as_bytes())?;

    println!("Running VM...");
    let end = vm.run();
    dbg!(end);

    let values: Vec<_> = vm.code.map.values().copied().collect();

    println!("Starting p-code generation and file writing...");
    Ok(HighPcodeGenerator {
        vm,
        values: values.into_iter(),
    })
}
