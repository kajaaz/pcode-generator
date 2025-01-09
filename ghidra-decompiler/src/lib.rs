#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// scary!
#![allow(improper_ctypes)]
pub mod sys {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use std::{cmp, fs::File, mem::MaybeUninit, os::unix::fs::FileExt, pin::Pin};

use goblin::elf::Elf;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("ghidra-decompiler/src/wrapper.hh");
        type PcodeDecoder;

        unsafe fn new_pcode_decoder(specfile: &str, elf: *mut u8, base_addr: u64) -> UniquePtr<PcodeDecoder>;
        unsafe fn decode_addr(&self, addr: u64, instr_len: *mut u64) -> Result<String>;
    }

    extern "Rust" {
        unsafe fn load_fill(elf_ptr: *mut u8, data: *mut u8, size: u32, addr: u64);
    }
}

pub struct PcodeDecoder<'a> {
    decoder: cxx::UniquePtr<ffi::PcodeDecoder>,
    file: &'a mut File,
    elf: &'a Elf<'a>,
}

impl<'a> PcodeDecoder<'a> {
    pub fn new(
        spec_file: &str,
        file: &'a mut File,
        elf: &'a Elf<'a>,
        base_addr: u64,
    ) -> Result<Pin<Box<Self>>, Box<dyn std::error::Error>> {
        unsafe {
            let mut slf = Box::pin(MaybeUninit::zeroed());
            let slf_ptr = core::mem::transmute(&mut *slf);
            let decoder = ffi::new_pcode_decoder(spec_file, slf_ptr, base_addr);
            slf.write(Self { decoder, file, elf });
            Ok(core::mem::transmute(slf))
        }
    }

    pub fn decode_addr(&mut self, addr: u64) -> Result<(String, u64), Box<dyn std::error::Error>> {
        let mut len = 0;
        let output = unsafe { self.decoder.decode_addr(addr, &mut len)? };
        Ok((output, len))
    }

    pub fn load_fill(&mut self, data: &mut [u8], addr: u64) {
        let start = addr;
        let end = addr + u64::try_from(data.len()).unwrap();
    
        data.fill(0);
    
        for ph in self.elf.program_headers.iter() {
            if ph.p_type != goblin::elf::program_header::PT_LOAD {
                continue; // Skip non-loadable segments
            }
    
            let segment_start = ph.p_vaddr;
            let segment_end = segment_start + ph.p_filesz; // Use p_filesz for file data
    
            // Check for intersection between requested address range and segment
            if start < segment_end && end > segment_start {
                let intersection_start = cmp::max(start, segment_start);
                let intersection_end = cmp::min(end, segment_end);
                let file_offset = ph.p_offset + (intersection_start - segment_start);
                self.file
                    .read_exact_at(
                        &mut data[(intersection_start - start) as usize
                            ..(intersection_end - start) as usize],
                        file_offset,
                    )
                    .unwrap();
            }
        }
    }         
}

pub unsafe fn load_fill(elf_ptr: *mut u8, data: *mut u8, size: u32, addr: u64) {
    let decoder: &mut PcodeDecoder<'_> = core::mem::transmute(elf_ptr);
    let data = core::slice::from_raw_parts_mut(data, usize::try_from(size).unwrap());
    (*decoder).load_fill(data, addr);
}