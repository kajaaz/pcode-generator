#include <cstdint>

#include <string.h>

#include <iostream>

#include "wrapper.hh"

using namespace std;
using namespace ghidra;

// This is the only important method for the LoadImage. It returns bytes from
// the static array depending on the address range requested
void MyLoadImage::loadFill(uint1 *ptr, int4 size, const Address &addr) {
  uintb start = addr.getOffset();
  uintb max = min(endaddr, baseaddr + data.size());
  for (int4 i = 0; i < size; ++i) {
    uintb curoff = start + i;
    if ((curoff < baseaddr) || (curoff >= max)) {
      ptr[i] = 0;
      continue;
    }
    uintb diff = curoff - baseaddr;
    ptr[i] = data[diff];
  }
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
PcodeDecoder::PcodeDecoder(string &specfile, vector<uint1> data, uintb base, uintb end)
    : loader(base, data), sleigh(&loader, &context) {
  // Read sleigh file into DOM
  Element *sleighroot = docstorage.openDocument(specfile)->getRoot();
  docstorage.registerTag(sleighroot);
  sleigh.initialize(docstorage); // Initialize the translator

  // Now that context symbol names are loaded by the translator
  // we can set the default context
  context.setVariableDefault("addrsize", 1); // Address size is 32-bit
  context.setVariableDefault("opsize", 1);   // Operand size is 32-bit
};

// -------------------------------------
//
// Functions called direclty from rust

rust::String PcodeDecoder::decode_addr(uint64_t addr_in,
                                       uint64_t *instr_len) const {
  Address addr(sleigh.getDefaultCodeSpace(), addr_in);
  PcodeRawOut emit; // Set up the pcode dumper
  int4 length;      // Number of bytes of each machine instruction
  *instr_len = sleigh.oneInstruction(emit, addr); // Translate instruction
  return string(emit.getPcode());
}

unique_ptr<PcodeDecoder> new_pcode_decoder(rust::Str specfile_str, rust::Str parsefile_str, uintb base_addr, uintb end_addr) {
  std::string specfile(specfile_str);
  std::string parsefile(parsefile_str);

  // Open the file, in raw binary mode for now
  std::ifstream file(parsefile, ios::binary);
  if (!file) {
    throw std::runtime_error("Unable to open file");
  }

  // Determine the size to read
  file.seekg(0, ios::end);
  int size = file.tellg();
  file.seekg(0, ios::beg);

  // Resize the buffer and read the data
  vector<uint1> data(size);
  if (!file.read(reinterpret_cast<char *>(data.data()), size)) {
    throw std::runtime_error("Error reading file");
  }
  data.resize(size);
  file.close();

  // static initializations
  AttributeId::initialize();
  ElementId::initialize();

  // Set up the assembler/pcode-translator
  return unique_ptr<PcodeDecoder>(new PcodeDecoder(specfile, data, base_addr, end_addr));
}


