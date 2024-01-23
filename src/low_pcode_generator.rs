use crate::pcode_generator;
use std::io::Write;

pub fn generate_low_pcode(filename: &str) {
    // TODO this should be a configurable path, not a constant value
    const PROJECT: &str = env!("CARGO_MANIFEST_DIR");
    let spec_file = format!("{PROJECT}/src/specfiles/x86.sla");
    let decoder = ghidra_decompiler::PcodeDecoder::new(&spec_file, filename).unwrap();
    let mut output_file = pcode_generator::create_output_file(filename, "low")
        .expect("Unable to create the output file");

    // TODO implement a detection to identify the start and end of a
    // decompilation block.
    // for now always decompile 10 instructions
    let mut addr = 0;
    for _i in 0..10 {
        let (pcode, instruction_len) = decoder.decode_addr(addr).unwrap();
        if let Err(e) = writeln!(output_file, "0x{addr:x}\n{pcode}") {
            eprintln!("Failed to write to output file: {:?}", e);
            return;
        }
        addr += instruction_len;
    }
}
