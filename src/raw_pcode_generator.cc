/* ###
 * IP: GHIDRA
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 * 
 *      http://www.apache.org/licenses/LICENSE-2.0
 * 
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * 
 * Modified by @kajaaz in January 2024
 */

#include "raw_pcode_generator.hh"
#include <iostream>
#include <sstream>

namespace ghidra {

void AssemblyEmitExtend::dump(const Address &addr, const std::string &mnem, const std::string &body) {
    std::cout << addr.getShortcut() << ": " << mnem << " " << body << std::endl;
}

PcodeEmitExtend::~PcodeEmitExtend() {
    if (outputFile.is_open()) {
        outputFile.close();
    }
}

void PcodeEmitExtend::dump(const Address &addr, OpCode opc, VarnodeData *outvar, VarnodeData *vars, int4 isize) {
    PcodeOpRaw op;
    bool isUnary = isUnaryOperation(opc);         // Use isUnary to determine behavior type  
    op.setBehavior(new OpBehavior(opc, isUnary)); //< true= use unary interfaces,  false = use binary
    op.setSeqNum(addr, 0);

    if (outvar != nullptr) {
        op.setOutput(outvar);
    }

    for (int4 i = 0; i < isize; ++i) {
        op.addInput(vars + i);
    }

    pcodeOps.push_back(op);
}

const std::vector<PcodeOpRaw>& PcodeEmitExtend::getPcodeOps() const {
    return pcodeOps;
}

bool isUnaryOperation(OpCode opc) {
    switch (opc) {
        case CPUI_COPY:
        case CPUI_INT_ZEXT:
        case CPUI_INT_SEXT:
        case CPUI_INT_2COMP:
        case CPUI_INT_NEGATE:
        case CPUI_BOOL_NEGATE:
        case CPUI_FLOAT_NEG:
        case CPUI_FLOAT_ABS:
        case CPUI_FLOAT_SQRT:
        case CPUI_FLOAT_INT2FLOAT:
        case CPUI_FLOAT_FLOAT2FLOAT:
        case CPUI_FLOAT_TRUNC:
        case CPUI_FLOAT_CEIL:
        case CPUI_FLOAT_FLOOR:
        case CPUI_FLOAT_ROUND:
        case CPUI_POPCOUNT:
        case CPUI_LZCOUNT:
        case CPUI_FLOAT_NAN: 
        case CPUI_INDIRECT: 
        case CPUI_CAST: 
        case CPUI_NEW: 
            return true;
        default:
            return false;
    }
}

// Constructor for RawLoadImagePcode
RawLoadImagePcode::RawLoadImagePcode(const std::string &f) : LoadImage(f) {
  vma = 0;
  thefile = nullptr;
  spaceid = nullptr;
  filesize = 0;
}

// Destructor for RawLoadImagePcode
RawLoadImagePcode::~RawLoadImagePcode() {
  if (thefile != nullptr) {
    thefile->close();
    delete thefile;
  }
}

// Open the file for reading
void RawLoadImagePcode::open() {
  if (thefile != nullptr)
    throw LowlevelError("loadimage is already open");

  thefile = new std::ifstream(filename.c_str());
  if (!(*thefile)) {
    std::string errmsg = "Unable to open raw image file: " + filename;
    throw LowlevelError(errmsg);
  }

  thefile->seekg(0, std::ios::end);
  filesize = thefile->tellg();
}


// Get the architecture type
std::string RawLoadImagePcode::getArchType() const {
  return "x86";
}

// Get the filesize
uintb RawLoadImagePcode::getFileSize() const {
  return filesize;
}

// Adjust VMA (Virtual Memory Address)
void RawLoadImagePcode::adjustVma(long adjust) {
  adjust = AddrSpace::addressToByte(adjust, spaceid->getWordSize());
  vma += adjust;
}

// Load data into a buffer
void RawLoadImagePcode::loadFill(uint1 *ptr, int4 size, const Address &addr) {
  uintb curaddr = addr.getOffset();
  uintb offset = 0;
  uintb readsize;

  curaddr -= vma; // Get relative offset of the first byte

  while (size > 0) {
    if (curaddr >= filesize) {
      if (offset == 0) // Initial address not within the file
        break;
      memset(ptr + offset, 0, size); // Fill out the rest of the buffer with 0
      return;
    }
    readsize = size;
    if (curaddr + readsize > filesize) // Adjust to the biggest possible read
      readsize = filesize - curaddr;
    thefile->seekg(curaddr);
    thefile->read(reinterpret_cast<char *>(ptr + offset), readsize);
    offset += readsize;
    size -= readsize;
    curaddr += readsize;
  }

  if (size > 0) {
    std::ostringstream errmsg;
    errmsg << "Unable to load " << size << " bytes at " << addr.getShortcut();
    addr.printRaw(errmsg);
    throw DataUnavailError(errmsg.str());
  }
}

// Function used by main.rs to generate the raw pcode
extern "C" const char* generate_raw_pcode(const char *filename) {
  if (filename == nullptr) {
    return strdup("Filename is null");
  }
  try {
    RawLoadImagePcode loader(filename);
    ContextInternal context;
    Sleigh trans(&loader, &context);

    std::string sleighSpecFile = "x86.slaspec"; // Update this path
    DocumentStorage docstorage;
    Element *sleighroot = docstorage.openDocument(sleighSpecFile)->getRoot();
    docstorage.registerTag(sleighroot);
    trans.initialize(docstorage);

    Address startAddr(trans.getDefaultCodeSpace(), 0);
    Address endAddr(trans.getDefaultCodeSpace(), loader.getFileSize());

        Address addr = startAddr;
        while (addr < endAddr) {
            try {
                AssemblyEmitExtend assememit;
                trans.printAssembly(assememit, addr);

                PcodeEmitExtend pcodeemit;
                trans.oneInstruction(pcodeemit, addr);

                // Output the pcode
                const std::vector<PcodeOpRaw> &ops = pcodeemit.getPcodeOps();
                for (const auto &op : ops) {
                    // Format and output the pcode operation
                    std::cout << "PcodeOp at " << op.getSeqNum().getAddr().getShortcut() << ", Opcode: " << op.getOpcode();
                    if (op.getOutput() != nullptr) {
                        std::cout << ", Output: " << op.getOutput()->getAddr().getShortcut();
                    }
                    std::cout << std::endl;
                }

                addr = addr + trans.instructionLength(addr);
            } catch (const BadDataError &e) {
                std::cerr << "Bad data at address " << addr.getShortcut() << ": " << e.explain << std::endl;
            }
        }
        return nullptr; // Success, no error
    } catch (const std::exception &e) {
        // Convert the C++ exception to a Rust panic
        std::string error_msg = "C++ Exception: " + std::string(e.what());
        throw std::runtime_error(error_msg);
        return strdup(e.what()); // Error, return the error message
    }
  }
} // End namespace ghidra
