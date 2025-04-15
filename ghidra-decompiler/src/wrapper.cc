#include <cstdint>
#include <exception>
#include <iostream>
#include <string.h>
#include <sstream>

#include "ghidra-decompiler/src/lib.rs.h"
#include "wrapper.hh"

using namespace std;
using namespace ghidra;

// This is the only important method for the LoadImage. It returns bytes from
// the static array depending on the address range requested
// In Ghidra the addresses are 'absolute', whereas in this loadfill function, the addresses are considered without the base address
void MyLoadImage::loadFill(uint1 *ptr, int4 size, const Address &addr) {
  load_fill(rust_dec, ptr, size, addr.getOffset() - base_addr);
}

// -------------------------------
//
// These are the classes/routines relevant to printing a pcode translation
// Modified function to handle spaces ID for LOAD and STORE
void print_vardata(ostream &s, OpCode opc, const VarnodeData &data, bool isFirstParam) {
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
PcodeDecoder::PcodeDecoder(string &specfile, uint8_t *rust_dec, rust::cxxbridge1::u64 base_addr) 
  : loader(rust_dec, base_addr), sleigh(&loader, &context), base_addr(base_addr) { 
  
  std::cerr << "[CONSTRUCTOR] specfile='" << specfile
      << "' base_addr=0x" << std::hex << base_addr << std::endl;

  // 1) Wrap the .sla path in <sleigh>...</sleigh>
  std::string wrappedXml = "<sleigh>" + specfile + "</sleigh>";
  std::istringstream sleighSpecStream(wrappedXml);

  try {
    // 2) Parse the tiny XML doc
    Element *sleighroot = docstorage.parseDocument(sleighSpecStream)->getRoot();
    docstorage.registerTag(sleighroot);
  
    // DEBUG: Print the root element name and text
    // std::cerr << "[CONSTRUCTOR] [DEBUG] sleighroot->getName():   "
    //           << sleighroot->getName() << std::endl;
    // std::cerr << "[CONSTRUCTOR] [DEBUG] sleighroot->getContent(): "
    //           << sleighroot->getContent() << std::endl;
  
    // 3) Let the Sleigh engine load the compiled .sla
    sleigh.initialize(docstorage);
    std::cerr << "[CONSTRUCTOR] Sleigh initialization SUCCESS" << std::endl;
  
  } catch (const ghidra::LowlevelError &err) {
    // Use 'err.explain' instead of 'err.what()'
    std::cerr << "[CONSTRUCTOR] Sleigh initialization FAILED: "
              << err.explain << std::endl;
    throw; // re-throw or handle as needed
  }
  
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

  return;
}

// -------------------------------------
//
// Functions called directly from rust
rust::String PcodeDecoder::decode_addr(uint64_t addr_in, uint64_t *instr_len) const {
  Address addr(sleigh.getDefaultCodeSpace(), addr_in + this->base_addr);
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

unique_ptr<PcodeDecoder> new_pcode_decoder(rust::Str specfile_str, uint8_t *rust_dec, rust::cxxbridge1::u64 base_addr) {
  std::string specfile(specfile_str);

  AttributeId::initialize();
  ElementId::initialize();

  return unique_ptr<PcodeDecoder>(new PcodeDecoder(specfile, rust_dec, base_addr));
}