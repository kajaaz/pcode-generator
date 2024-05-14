#include <cstdint>
#include <exception>
#include <iostream>
#include <string.h>

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
// Modified function to handle spaces ID for LOAD and STORE
static void print_vardata(ostream &s, OpCode opc, const VarnodeData &data, bool isFirstParam) {
  if ((opc == CPUI_LOAD || opc == CPUI_STORE) && isFirstParam && data.space->getType() == IPTR_CONSTANT) {
    AddrSpace *spc = data.getSpaceFromConst();
    s << '(' << spc->getName() << ')';
  } else {
    s << '(' << data.space->getName() << ',';
    data.space->printOffset(s, data.offset);
    s << ',' << dec << data.size << ')';
  }
}

// Here is a simple class for emitting pcode. We simply dump an appropriate
// string representation straight to standard out.
class PcodeRawOut : public PcodeEmit {
  std::ostringstream pcodeStream;

public:
  void dump(const Address &addr, OpCode opc, VarnodeData *outvar,
            VarnodeData *vars, int4 isize) {
    if (outvar != nullptr) {
      print_vardata(pcodeStream, opc, *outvar, false);
      pcodeStream << " = ";
    }
    pcodeStream << get_opname(opc);
    for (int4 i = 0; i < isize; ++i) {
      pcodeStream << ' ';
      print_vardata(pcodeStream, opc, vars[i], (i == 0));
    }
    pcodeStream << '\n';
  }
  std::string getPcode() const { return pcodeStream.str(); }
};

// TODO configure a base address or just implement an elf reader instead of
// using a raw binary
PcodeDecoder::PcodeDecoder(string &specfile, uint8_t *rust_dec)
    : loader(rust_dec), sleigh(&loader, &context) {
  // Read sleigh file into DOM
  Element *sleighroot = docstorage.openDocument(specfile)->getRoot();
  docstorage.registerTag(sleighroot);
  sleigh.initialize(docstorage); // Initialize the translator

  // Now that context symbol names are loaded by the translator
  // we can set the default context
  // x86_64 64bits: longMode = 1, bit64 = 1, addrsize = 2, opsize = 1
  // x86_64 32bits: longMode = 0, bit64 = 1, addrsize = 1, opsize = 1
  // x86 32bits   : longMode = 0, bit64 = 0, addrsize = 1, opsize = 0
  // x86 16bits   : longMode = 0, bit64 = 0, addrsize = 0, opsize = 0
  context.setVariableDefault("longMode", 1);   // Enable 64-bit mode
  context.setVariableDefault("bit64", 1);      // 64-bit addressing
  context.setVariableDefault("addrsize", 2);   // Address size is 64-bit
  context.setVariableDefault("opsize", 1);     // Operand size is 64-bit
}

// -------------------------------------
//
// Functions called directly from rust

rust::String PcodeDecoder::decode_addr(uint64_t addr_in, uint64_t *instr_len) const {
  Address addr(sleigh.getDefaultCodeSpace(), addr_in);
  PcodeRawOut emit;
  int4 length;

  try {
    *instr_len = sleigh.oneInstruction(emit, addr);
  } catch (const LowlevelError &e) {
    std::cerr << "LowlevelError at address 0x" << std::hex << addr_in << ": " << e.explain << "\n";
    throw runtime_error("Error: Disassembly failed due to LowlevelError: " + e.explain);
  } catch (const std::exception &e) {
    std::cerr << "Standard exception at address 0x" << std::hex << addr_in << ": " << e.what() << "\n";
    throw runtime_error("Error: Disassembly failed due to a standard exception: " + string(e.what()));
  } catch (...) {
    std::cerr << "Unknown error at address 0x" << std::hex << addr_in << "\n";
    throw runtime_error("Error: Disassembly failed due to an unknown error.");
  }
  return string(emit.getPcode());
}

void PcodeDecoder::updateContext(void) {
    context.setVariableDefault("longMode", 1); // Enable 64-bit mode
    context.setVariableDefault("addrsize", 2); // Address size is 64-bit
    context.setVariableDefault("opsize", 2);   // Operand size is 64-bit
}

unique_ptr<PcodeDecoder> new_pcode_decoder(rust::Str specfile_str,
                                           uint8_t *rust_dec) {
  std::string specfile(specfile_str);

  AttributeId::initialize();
  ElementId::initialize();

  return unique_ptr<PcodeDecoder>(new PcodeDecoder(specfile, rust_dec));
}
