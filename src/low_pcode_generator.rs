use crate::pcode_generator;
use goblin::elf::Elf;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};
use std::process::Command;

// Helper function to execute `nm` and parse its output
fn parse_nm_output(filename: &str) -> io::Result<HashMap<u64, u64>> {
    let output = Command::new("nm")
        .arg("--print-size")
        .arg(filename)
        .output()?;

    if !output.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to execute nm command",
        ));
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    let symbols = output_str
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 && (parts[2].starts_with('t') || parts[2].starts_with('T')) {
                let start_addr = u64::from_str_radix(parts[0], 16).ok()?;
                let size = u64::from_str_radix(parts[1], 16).ok()?;
                Some((start_addr, size))
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>();

    Ok(symbols)
}

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
    // let sections = elf.section_headers.iter()
    //     .filter(|s| s.is_executable())
    //     .map(|s| {
    //         let start_addr = s.sh_addr;
    //         let end_addr = s.sh_addr + s.sh_size;
    //         let file_offset = s.sh_offset; 
    //         (start_addr, end_addr, file_offset)
    //     })
    //     .collect::<Vec<_>>();

    // Parse nm output to get executable symbols with their sizes
    let symbols = parse_nm_output(filename)?;

    for (start_addr, size) in symbols {
        let end_addr = start_addr + size;
        //println!("Symbol Start Address: 0x{:x}, Size: 0x{:x}", start_addr, size);
        let mut addr = start_addr;

        while addr < end_addr {
            match decoder.decode_addr(addr) {
                Ok((pcode, instruction_len)) => {
                    write!(output_file, "0x{:x}\n{}", addr, pcode)?;
                    addr += instruction_len;
                },
                Err(e) => {
                    eprintln!("Error at address 0x{:x}: {}", addr, e);

                    // Stop processing further on error
                    // return Err(io::Error::new(io::ErrorKind::Other, "Disassembly error, stopping."));

                    // Skipping problematic instructions (assuming it is data from crypto lib)
                    addr += 1; 
                }
            }
        }
    }

    Ok(())
}

