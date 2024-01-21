pub fn generate_low_pcode(filename: &str) {
    // TODO this should be a configurable path, not a constant value
    const PROJECT: &str = env!("CARGO_MANIFEST_DIR");
    let spec_file = format!("{PROJECT}/src/specfiles/x86.sla");
    let decoder = ghidra_decompiler::PcodeDecoder::new(&spec_file, filename).unwrap();
    // TODO implement a detection to identify the start and end of a
    // decompilation block.
    // for now always decompile 10 instructions
    let mut addr = 0;
    for _i in 0..10 {
        let (pcode, instruction_len) = decoder.decode_addr(addr).unwrap();
        println!("instruction at 0x{addr:x} len {instruction_len}:\n{pcode}");
        addr += instruction_len;
    }
}
