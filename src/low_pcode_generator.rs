pub fn generate_low_pcode(
    bin_file: &str,
) -> Result<impl Iterator<Item = String>, Box<dyn std::error::Error>> {
    // TODO this should be a configurable path, not a constant value
    const PROJECT: &str = env!("CARGO_MANIFEST_DIR");
    let spec_file = format!("{PROJECT}/src/specfiles/x86.sla");
    let decoder = ghidra_decompiler::PcodeDecoder::new(&spec_file, bin_file)?;
    // TODO implement a detection to identify the start and end of a
    // decompilation block.
    // for now always decompile 10 instructions
    let mut addr = 0;
    Ok((0..10).map(move |_| {
        let (pcode, instruction_len) = decoder.decode_addr(addr).unwrap();
        addr += instruction_len;
        pcode
    }))
}
