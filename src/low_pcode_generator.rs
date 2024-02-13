use crate::pcode_generator;
use std::io::Write;

pub fn generate_low_pcode(filename: &str) {

    // TO BE MODIFIED
    // -------------------------------
    let base_addr = 0x400f9c;  //  Hardcoded base address - TODO: improve UX
    let end_addr = 0x55e8b0;   // Hardcoded end address - TODO: improve UX
    // -------------------------------
    // test tests/calculus/calculus with these values
    // let base_addr = 0x0000000000401000; 
    // let end_addr = 0x000000000048FFD8; 
    //  
    // for .text of geth
    // let base_addr = 0x0000000000414840; 
    // let end_addr = 0x000000000015E24DF;

    // TODO this should be a configurable path, not a constant value
    const PROJECT: &str = env!("CARGO_MANIFEST_DIR");
    let spec_file = format!("{PROJECT}/src/specfiles/x86-64.sla");
    let decoder = ghidra_decompiler::PcodeDecoder::new(&spec_file, filename, base_addr, end_addr).unwrap();
    let mut output_file = pcode_generator::create_output_file(filename, "low")
        .expect("Unable to create the output file");

    let mut addr = base_addr;
    while addr < end_addr {
        let (pcode, instruction_len) = decoder.decode_addr(addr).unwrap();
        
        // Uncomment this line if you want to print the corresponding address to the pcode
        if let Err(e) = write!(output_file, "0x{:x}\n{}", addr, pcode) {
            eprintln!("Failed to write to output file: {:?}", e);
            return;
        }
        //if let Err(e) = write!(output_file, "{}", pcode) {
        //    eprintln!("Failed to write to output file: {:?}", e);
        //    return;
        //}
        addr += instruction_len;
    }
}
