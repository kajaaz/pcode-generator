use std::{fs::File, os::unix::fs::FileExt, pin::Pin, mem::MaybeUninit};

use goblin::elf::Elf;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("ghidra-decompiler/src/wrapper.hh");
        type PcodeDecoder;

        unsafe fn new_pcode_decoder(specfile: &str, elf: *mut u8) -> UniquePtr<PcodeDecoder>;
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
    ) -> Result<Pin<Box<Self>>, Box<dyn std::error::Error>> {
        unsafe {
            let mut slf = Box::pin(MaybeUninit::zeroed());
            let slf_ptr = core::mem::transmute(&mut *slf);
            let decoder = ffi::new_pcode_decoder(spec_file, slf_ptr);
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
        // TODO deal with reads between sections
        let Some((data_offset, file_offset, read_size)) =
            self.elf.section_headers.iter().find_map(|header| {
                let section_start = header.sh_addr;
                let section_end = section_start + header.sh_size;
                let section_range = section_start..section_end;
                (section_range.contains(&start)
                    || section_range.contains(&end)
                    || (start..end).contains(&section_start)
                    || (start..end).contains(&section_end))
                .then(|| {
                    let fill_before = section_start.saturating_sub(start);
                    let fill_after = end.saturating_sub(section_end);
                    let skip_section = start.saturating_sub(section_start);
                    let offset = header.sh_offset + skip_section;
                    let section_size_left = header.sh_size - skip_section;
                    let size_left = (u64::try_from(data.len()).unwrap() - fill_before) - fill_after;
                    let size = section_size_left.min(size_left);
                    (
                        usize::try_from(fill_before).unwrap(),
                        offset,
                        usize::try_from(size).unwrap(),
                    )
                })
            })
        else {
            data.fill(0);
            return;
        };
        data[0..data_offset].fill(0);
        let rest = &mut data[data_offset..];
        self.file
            .read_exact_at(&mut rest[0..read_size], file_offset)
            .unwrap();
        rest[read_size..].fill(0);
    }
}

pub unsafe fn load_fill(elf_ptr: *mut u8, data: *mut u8, size: u32, addr: u64) {
    let decoder: &mut PcodeDecoder<'_> = core::mem::transmute(elf_ptr);
    let data = core::slice::from_raw_parts_mut(data, usize::try_from(size).unwrap());
    (*decoder).load_fill(data, addr);
}
