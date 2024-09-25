use goblin::elf::{self, Elf};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, Read, Write};
use std::process::Command;

use crate::pcode_generator;

// Extract symbols with Pyhidra
fn extract_symbols_with_pyhidra(filename: &str) -> io::Result<BTreeMap<u64, (String, u64)>> {
    let output = Command::new("python3")
        .arg("src/analyze_binary.py")  // Call the Python script
        .arg(filename)
        .output()?;

    // Check if the process exited with an error
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error running Pyhidra analysis: {}", stderr);
        return Err(io::Error::new(io::ErrorKind::Other, "Failed to run Pyhidra analysis"));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    let mut symbols = BTreeMap::new();

    // Parse the output from the Python script
    for line in stdout.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 3 {
            continue;
        }
        let address = u64::from_str_radix(parts[0], 16).unwrap_or(0);
        let name = parts[1].to_string();
        let size = parts[2].parse::<u64>().unwrap_or(0);
        symbols.insert(address, (name, size));
    }

    Ok(symbols)
}

pub fn generate_low_pcode(filename: &str) -> io::Result<()> {
    // Read the binary file
    let mut f = File::open(filename)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    // Parse the ELF file (needed by the PcodeDecoder)
    let elf = Elf::parse(&buffer).map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    // Extract symbols using Pyhidra (get additional function information)
    let symbols = extract_symbols_with_pyhidra(filename)?;

    // Configuration
    const PROJECT: &str = env!("CARGO_MANIFEST_DIR");
    let spec_file = format!("{}/src/specfiles/x86-64.sla", PROJECT);

    // Pass the ELF data structure to the decoder, alongside the symbols
    let mut decoder = ghidra_decompiler::PcodeDecoder::new(&spec_file, &mut f, &elf)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    let mut output_file = pcode_generator::create_output_file(filename, "low")
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    // Iterate over symbols and decode instructions using the decoder
    for (start_addr, (_name, size)) in symbols {
        let end_addr = start_addr + size;
        let mut addr = start_addr;

        while addr < end_addr {
            match decoder.decode_addr(addr) {
                Ok((pcode, instruction_len)) => {
                    write!(output_file, "0x{:x}\n{}", addr, pcode)?;
                    addr += instruction_len;
                },
                Err(e) => {
                    eprintln!("Error at address 0x{:x}: {}", addr, e);
                    return Err(io::Error::new(io::ErrorKind::Other, "Disassembly error, stopping."));
                }
            }
        }
    }

    Ok(())
}
