#include <cstdint>

#include <string.h>

#include <iostream>

#include <exception>

#include "ghidra-decompiler/src/lib.rs.h"
#include "wrapper.hh"

using namespace std;
using namespace ghidra;

// This is the only important method for the LoadImage. It returns bytes from
// the static array depending on the address range requested
void MyLoadImage::loadFill(uint1 *ptr, int4 size, const Address &addr) {
  load_fill(rust_dec, ptr, size, addr.getOffset());
}

// -------------------------------
//
// These are the classes/routines relevant to printing a pcode translation

// Here is a simple class for emitting pcode. We simply dump an appropriate
// string representation straight to standard out.
class PcodeRawOut : public PcodeEmit {
  std::ostringstream pcodeStream;

public:
  virtual void dump(const Address &addr, OpCode opc, VarnodeData *outvar,
                    VarnodeData *vars, int4 isize);
  std::string getPcode() const { return pcodeStream.str(); }
};

static void print_vardata(ostream &s, VarnodeData &data)

{
  s << '(' << data.space->getName() << ',';
  data.space->printOffset(s, data.offset);
  s << ',' << dec << data.size << ')';
}

void PcodeRawOut::dump(const Address &addr, OpCode opc, VarnodeData *outvar,
                       VarnodeData *vars, int4 isize) {
  // Write to pcodeStream instead of cout
  if (outvar != nullptr) {
    print_vardata(pcodeStream, *outvar);
    pcodeStream << " = ";
  }
  pcodeStream << get_opname(opc);
  // Possibly check for a code reference or a space reference
  for (int4 i = 0; i < isize; ++i) {
    pcodeStream << ' ';
    print_vardata(pcodeStream, vars[i]);
  }
  pcodeStream << '\n';
}

// TODO configure a base address or just implement a elf reader instead of
// using a raw binary
PcodeDecoder::PcodeDecoder(string &specfile, uint8_t *rust_dec)
    : loader(rust_dec), sleigh(&loader, &context) {
  // Read sleigh file into DOM
  Element *sleighroot = docstorage.openDocument(specfile)->getRoot();
  docstorage.registerTag(sleighroot);
  sleigh.initialize(docstorage); // Initialize the translator

  // Now that context symbol names are loaded by the translator
  // we can set the default context
  context.setVariableDefault("longMode", 1); // Enable 64-bit mode
  context.setVariableDefault("addrsize", 2); // Address size is 64-bit
  context.setVariableDefault("opsize", 2);   // Operand size is 64-bit
};

// -------------------------------------
//
// Functions called direclty from rust

rust::String PcodeDecoder::decode_addr(uint64_t addr_in,
                                       uint64_t *instr_len) const {
  Address addr(sleigh.getDefaultCodeSpace(), addr_in);
  PcodeRawOut emit; // Set up the pcode dumper
  int4 length;      // Number of bytes of each machine instruction

  try {
    *instr_len = sleigh.oneInstruction(emit, addr); // Translate instruction
  } catch (const ghidra::LowlevelError &e) {
    throw runtime_error("Error: Disassembly failed due to LowlevelError: " +
                        e.explain);
  } catch (const std::exception &e) {
    throw runtime_error(
        string("Error: Disassembly failed due to a standard exception: ") +
        e.what());
  } catch (...) {
    throw runtime_error("Error: Disassembly failed due to an unknown error.");
  }
  return string(emit.getPcode());
}

unique_ptr<PcodeDecoder> new_pcode_decoder(rust::Str specfile_str,
                                           uint8_t *rust_dec) {
  std::string specfile(specfile_str);

  // static initializations
  AttributeId::initialize();
  ElementId::initialize();

  // Set up the assembler/pcode-translator
  return unique_ptr<PcodeDecoder>(new PcodeDecoder(specfile, rust_dec));
}
