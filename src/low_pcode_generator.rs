use crate::pcode_generator;
use std::io::Write;

pub fn generate_low_pcode(filename: &str) {

    // TO BE MODIFIED
    // -------------------------------
    let base_addr = 0x414840;  //  Hardcoded base address - TODO: improve UX
    let end_addr = 0x164DB20;   // Hardcoded end address - TODO: improve UX
    // -------------------------------

    // TODO this should be a configurable path, not a constant value
    const PROJECT: &str = env!("CARGO_MANIFEST_DIR");
    let spec_file = format!("{PROJECT}/src/specfiles/x86.sla");
    let decoder = ghidra_decompiler::PcodeDecoder::new(&spec_file, filename, base_addr, end_addr).unwrap();
    let mut output_file = pcode_generator::create_output_file(filename, "low")
        .expect("Unable to create the output file");

    let mut addr = base_addr;
    while addr < end_addr {
        let (pcode, instruction_len) = decoder.decode_addr(addr).unwrap();
        
        // Uncomment this line if you want to print the corresponding address to the pcode
        // if let Err(e) = writeln!(output_file, "0x{:x}\n{}", addr, pcode) {
        
        if let Err(e) = writeln!(output_file, "{}", pcode) {

            eprintln!("Failed to write to output file: {:?}", e);
            return;
        }
        addr += instruction_len;
    }
}
