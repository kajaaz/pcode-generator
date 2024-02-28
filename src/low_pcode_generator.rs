use crate::pcode_generator;
use goblin::elf::Elf;
use std::fs::File;
use std::io::{self, Read, Write};

pub fn generate_low_pcode(filename: &str) -> io::Result<()> {
    // Helper function to convert a goblin error to io::Error
    let to_io_error = |e: goblin::error::Error| io::Error::new(io::ErrorKind::Other, e.to_string());

    // Read the binary file
    let mut f = File::open(filename)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    // Parse the ELF file
    let elf = Elf::parse(&buffer).map_err(to_io_error)?;

    // Adjusted for dynamic addresses
    const PROJECT: &str = env!("CARGO_MANIFEST_DIR");
    let spec_file = format!("{}/src/specfiles/x86-64.sla", PROJECT);
    let mut decoder = ghidra_decompiler::PcodeDecoder::new(&spec_file, &mut f, &elf)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    let mut output_file = pcode_generator::create_output_file(filename, "low")
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    // determine base and end addresses for all executable sections
    let sections = elf
        .section_headers
        .iter()
        .filter(|ph| ph.is_executable())
        .map(|ph| (ph.sh_addr, ph.sh_addr + ph.sh_size));
    for (base_addr, end_addr) in sections {
        let mut addr = base_addr;
        while addr < end_addr {
            let (pcode, instruction_len) = decoder
                .decode_addr(addr)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

            // Write to output file
            // write!(output_file, "{}", pcode)
            write!(output_file, "0x{:x}\n{}", addr, pcode)?;
            addr += instruction_len;
        }
    }

    Ok(())
}
