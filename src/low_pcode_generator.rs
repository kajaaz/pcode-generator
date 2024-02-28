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

    // Configuration
    const PROJECT: &str = env!("CARGO_MANIFEST_DIR");
    let spec_file = format!("{}/src/specfiles/x86-64.sla", PROJECT);
    let mut decoder = ghidra_decompiler::PcodeDecoder::new(&spec_file, &mut f, &elf)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    let mut output_file = pcode_generator::create_output_file(filename, "low")
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    // determine base and end addresses for all executable sections
    let sections = elf.section_headers.iter()
        .filter(|s| s.is_executable())
        .map(|s| {
            let start_addr = s.sh_addr;
            let end_addr = s.sh_addr + s.sh_size;
            let file_offset = s.sh_offset; 
            (start_addr, end_addr, file_offset)
        })
        .collect::<Vec<_>>();

    for (start_addr, end_addr, file_offset) in sections {
        println!("Start Address: 0x{:x}", start_addr);
        println!("End Address: 0x{:x}", end_addr);
        let mut addr = start_addr;
        while addr < end_addr {
            match decoder.decode_addr(addr) {
                Ok((pcode, instruction_len)) => {
                    write!(output_file, "0x{:x}\n{}", addr, pcode)?;
                    addr += instruction_len;
                },
                Err(e) => {
                    eprintln!("Error at address 0x{:x}", addr);

                    // Calculate the actual file offset based on current addr
                    let current_file_offset = file_offset + (addr - start_addr);
                    let snippet = buffer.get(current_file_offset as usize..(current_file_offset + 16) as usize);
                    if let Some(bytes) = snippet {
                        eprintln!("Raw data at 0x{:x}: {:?}", addr, bytes);
                    } else {
                        eprintln!("Unable to retrieve raw data at 0x{:x}.", addr);
                    }
                    eprintln!("Error details: {}", e);
                    
                    // Stop processing further on error
                    return Err(io::Error::new(io::ErrorKind::Other, "Disassembly error, stopping."));
                }
            }
        }   
    }

    Ok(())
}


