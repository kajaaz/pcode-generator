#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("ghidra-decompiler/src/wrapper.hh");
        type PcodeDecoder;

        fn new_pcode_decoder(specfile: &str, parse_file: &str, base_addr: u64, end_addr: u64) -> UniquePtr<PcodeDecoder>;
        unsafe fn decode_addr(&self, addr: u64, instr_len: *mut u64) -> Result<String>;
    }
}

pub struct PcodeDecoder(cxx::UniquePtr<ffi::PcodeDecoder>);

impl PcodeDecoder {
    pub fn new(spec_file: &str, binary_file: &str, base_addr: u64, end_addr: u64) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self(ffi::new_pcode_decoder(spec_file, binary_file, base_addr, end_addr)))
    }

    pub fn decode_addr(&self, addr: u64) -> Result<(String, u64), Box<dyn std::error::Error>> {
        let mut len = 0;
        let output = unsafe { self.0.decode_addr(addr, &mut len)? };
        Ok((output, len))
    }
}
