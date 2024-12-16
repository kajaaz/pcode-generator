use goblin::elf::Elf;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use log::{info, warn};

use crate::pcode_generator;

pub fn generate_low_pcode(filename: &str) -> io::Result<()> {
    // Read the binary file into buffer
    let mut f = File::open(filename)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    // Parse the ELF file
    let elf = Elf::parse(&buffer).map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    // Get the output file path and create the file
    let output_file_path = pcode_generator::output_file_path(filename, "low")?;
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
                        warn!("Failed to map virtual address 0x{:x} to file offset", addr);
                        addr += 1; // Skip one byte and continue
                        continue;
                    }
                };

                let file_offset_usize = file_offset as usize;
                if file_offset_usize >= buffer.len() {
                    warn!("File offset 0x{:x} is beyond buffer size", file_offset);
                    break;
                }
                let max_bytes = buffer.len() - file_offset_usize;
                let bytes_to_read = std::cmp::min(16, max_bytes);
                let bytes = &buffer[file_offset_usize..file_offset_usize + bytes_to_read];
                info!(
                    "Bytes at 0x{:x} (file offset 0x{:x}): {:02x?}",
                    addr, file_offset, bytes
                );

                match decoder.decode_addr(addr) {
                    Ok((pcode, instruction_len)) => {
                        if instruction_len == 0 {
                            warn!("Instruction at 0x{:x} has zero length", addr);
                            addr += 1; 
                            continue;
                        }
                        info!("Instruction at 0x{:x} has length {}", addr, instruction_len);

                        writeln!(output_file, "0x{:x}", addr)?;
                        write!(output_file, "{}", pcode)?;

                        addr += instruction_len;
                    }
                    Err(e) => {
                        warn!("Error decoding instruction at 0x{:x}: {}", addr, e);
                        addr += 1; 
                    }
                }                
            }
        } else {
            info!("Section {} not found; skipping.", section_name);
        }
    }

    // Flush the file before appending calls from GOT
    drop(output_file);

    // Integrate GOT.PLT calls by running the Python script and appending its output
    integrate_got_plt_calls(filename, output_file_path.to_str().unwrap())?;

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

fn integrate_got_plt_calls(binary_path: &str, low_pcode_path: &str) -> io::Result<()> {
    let mut buffer = Vec::new();
    let mut file = File::open(binary_path)?;
    file.read_to_end(&mut buffer)?;

    let elf = Elf::parse(&buffer).map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    // Open output file for appending P-code
    let mut output_file = OpenOptions::new()
        .append(true)
        .open(low_pcode_path)?;

    // Locate the .got.plt section
    if let Some(section) = elf.section_headers.iter().find(|section| {
        if let Some(name) = elf.shdr_strtab.get_at(section.sh_name) {
            name == ".got.plt"
        } else {
            false
        }
    }) {
        let section_start = section.sh_addr;
        let section_size = section.sh_size as usize;
        let section_offset = section.sh_offset as usize;

        if section_offset + section_size <= buffer.len() {
            let section_data = &buffer[section_offset..section_offset + section_size];

            for (i, entry) in section_data.chunks(8).enumerate() {
                let entry_addr = section_start + (i * 8) as u64;
                let entry_value = u64::from_le_bytes(entry.try_into().unwrap_or_default());

                // Write P-code instructions for .got.plt entry
                writeln!(
                    output_file,
                    "0x{:x}\nCALL (ram,0x{:x},8)",
                    entry_addr, entry_value
                )?;
            }
        }
    } else {
        info!("No .got.plt section found; skipping.");
    }

    Ok(())
}
