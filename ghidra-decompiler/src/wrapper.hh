#ifndef WRAPPERHH
#define WRAPPERHH

#include <cstdint>
#include <memory>

#include "loadimage.hh"
#include "sleigh.hh"
#include "pcoderaw.hh"

#include "rust/cxx.h"

using namespace std;
using namespace ghidra;

// This is a tiny LoadImage class which feeds the executable bytes to the
// translator
class MyLoadImage : public LoadImage {
  uint8_t *rust_dec;
  rust::cxxbridge1::u64 base_addr;

public:
  MyLoadImage(uint8_t *rust_dec, rust::cxxbridge1::u64 base_addr) 
    : LoadImage("nofile"), rust_dec(rust_dec), base_addr(base_addr) {}
  virtual void loadFill(uint1 *ptr, int4 size, const Address &addr);
  virtual string getArchType(void) const { return "myload"; }
  virtual void adjustVma(long adjust) {}
};

class PcodeDecoder {
public:
  MyLoadImage loader;
  ContextInternal context;
  DocumentStorage docstorage;
  Sleigh sleigh;
  const rust::cxxbridge1::u64 base_addr;
  PcodeDecoder(string &specfile, uint8_t *rust_dec, rust::cxxbridge1::u64 base_addr);
  rust::String decode_addr(uint64_t addr, uint64_t *instr_len) const;
  void updateContext(void);
};

unique_ptr<PcodeDecoder> new_pcode_decoder(rust::Str specfile, uint8_t *rust_dec, rust::cxxbridge1::u64 base_addr);

#endif //WRAPPERHH