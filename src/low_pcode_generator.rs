use goblin::elf::Elf;
use std::fs::File;
use std::io::{self, Read, Write};
use log::{info, error};

use crate::pcode_generator;

pub fn generate_low_pcode(filename: &str) -> io::Result<()> {
    // Read the binary file into buffer
    let mut f = File::open(filename)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    // Parse the ELF file
    let elf = Elf::parse(&buffer).map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    // Open output file once
    let mut output_file = pcode_generator::create_output_file(filename, "low")
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    // Define sections to process
    let sections_to_process = [".text", ".plt"];
    for section_name in &sections_to_process {
        let section = elf.section_headers.iter().find(|section| {
            if let Some(name) = elf.shdr_strtab.get_at(section.sh_name) {
                name == *section_name
            } else {
                false
            }
        });

        if let Some(section) = section {
            let section_start = section.sh_addr;
            let section_size = section.sh_size as usize;
            let section_offset = section.sh_offset as usize;

            if section_offset + section_size > buffer.len() {
                error!("Section {} goes beyond buffer size", section_name);
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Invalid {} section size", section_name),
                ));
            }

            // Configuration for P-code generation
            const PROJECT: &str = env!("CARGO_MANIFEST_DIR");
            let spec_file = format!("{}/src/specfiles/x86-64.sla", PROJECT);

            // Initialize the decoder
            let mut decoder = ghidra_decompiler::PcodeDecoder::new(&spec_file, &mut f, &elf)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

            let mut addr = section_start;
            let end_addr = section_start + section_size as u64;

            while addr < end_addr {
                // Map the address to file offset
                let file_offset = match vaddr_to_offset(&elf, addr) {
                    Some(offset) => offset,
                    None => {
                        error!("Failed to map virtual address 0x{:x} to file offset", addr);
                        addr += 1; // Skip one byte and continue
                        continue;
                    }
                };

                let file_offset_usize = file_offset as usize;
                if file_offset_usize >= buffer.len() {
                    error!("File offset 0x{:x} is beyond buffer size", file_offset);
                    break;
                }
                let max_bytes = buffer.len() - file_offset_usize;
                let bytes_to_read = std::cmp::min(16, max_bytes);
                let bytes = &buffer[file_offset_usize..file_offset_usize + bytes_to_read];
                info!(
                    "Bytes at 0x{:x} (file offset 0x{:x}): {:02x?}",
                    addr, file_offset, bytes
                );

                // Decode the instruction
                match decoder.decode_addr(addr) {
                    Ok((pcode, instruction_len)) => {
                        if instruction_len == 0 {
                            error!("Instruction at 0x{:x} has zero length", addr);
                            addr += 1; // Skip one byte to avoid infinite loop
                            continue;
                        }
                        info!("Instruction at 0x{:x} has length {}", addr, instruction_len);

                        write!(output_file, "0x{:x}\n{}", addr, pcode)?;
                        addr += instruction_len;
                    }
                    Err(e) => {
                        error!("Error decoding instruction at 0x{:x}: {}", addr, e);
                        addr += 1; // Skip one byte and continue
                    }
                }
            }
        } else {
            error!("Failed to find {} section", section_name);
        }
    }

    Ok(())
}

fn vaddr_to_offset(elf: &Elf, vaddr: u64) -> Option<u64> {
    for ph in &elf.program_headers {
        if ph.p_type == goblin::elf::program_header::PT_LOAD {
            let vm_start = ph.p_vaddr;
            let vm_end = vm_start + ph.p_memsz;

            if vaddr >= vm_start && vaddr < vm_end {
                let offset = vaddr - vm_start + ph.p_offset;
                return Some(offset);
            }
        }
    }
    None
}
