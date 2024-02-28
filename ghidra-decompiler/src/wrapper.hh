#ifndef WRAPPERHH
#define WRAPPERHH

#include <cstdint>
#include <memory>

#include "loadimage.hh"
#include "sleigh.hh"

#include "rust/cxx.h"

using namespace std;
using namespace ghidra;

// This is a tiny LoadImage class which feeds the executable bytes to the
// translator
class MyLoadImage : public LoadImage {
  uint8_t *rust_dec;

public:
  MyLoadImage(uint8_t *rust_dec)
      : LoadImage("nofile"), rust_dec(rust_dec) {}
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
  PcodeDecoder(string &specfile, uint8_t *rust_dec);
  rust::String decode_addr(uint64_t addr, uint64_t *instr_len) const;
};

unique_ptr<PcodeDecoder> new_pcode_decoder(rust::Str specfile, uint8_t *rust_dec);

#endif //WRAPPERHH
