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

#include <iostream>
#include <sstream>
#include <vector>
#include <fstream>
#include <filesystem>

#include "../ghidra/Ghidra/Features/Decompiler/src/decompile/cpp/loadimage.hh"
#include "../ghidra/Ghidra/Features/Decompiler/src/decompile/cpp/sleigh.hh"
#include "../ghidra/Ghidra/Features/Decompiler/src/decompile/cpp/xml.hh"

using namespace std;

namespace ghidra {

// Custom LoadImage class for loading a binary file
class MyLoadImage : public LoadImage {
    uintb baseaddr;
    uintb endaddr;
    std::vector<uint1> data;

public:
    // Constructor: Initializes and loads a binary file into memory
    MyLoadImage(uintb ad, const std::string &filePath, uintb endAd) : LoadImage("nofile"), baseaddr(ad), endaddr(endAd) {
        
        // Open the file
        std::ifstream file(filePath, std::ios::binary);
        if (!file) {
            throw std::runtime_error("Unable to open file");
        }

        // Determine the size to read
        std::streamsize size = endaddr - baseaddr;
        if (size <= 0) {
            throw std::runtime_error("Invalid end address");
        }

        // Resize the buffer and read the data
        data.resize(size);
        file.seekg(baseaddr, std::ios::beg);
        if (!file.read(reinterpret_cast<char*>(data.data()), size)) {
            throw std::runtime_error("Error reading file");
        }
    }

    virtual void loadFill(uint1* ptr, int4 size, const Address& addr) override {
        uintb start = addr.getOffset();
        uintb max = baseaddr + (data.size() - 1);

        for (int4 i = 0; i < size; ++i) {
            uintb curoff = start + i;
            if ((curoff < baseaddr) || (curoff > max)) {
                ptr[i] = 0;
                continue;
            }
            ptr[i] = data[curoff - baseaddr];
        }
    }

    virtual string getArchType(void) const override { return "myload"; }
    virtual void adjustVma(long adjust) override { /* add content */ }

    uintb getBaseAddress() const { return baseaddr; }
    size_t getSize() const { return data.size(); }
};

class AssemblyRaw : public AssemblyEmit {
public:
  virtual void dump(const Address &addr,const string &mnem,const string &body) {
    addr.printRaw(cout);
    cout << ": " << mnem << ' ' << body << endl;
  }
};

class PcodeRawOut : public PcodeEmit {
  std::ostringstream pcodeStream;
public:
  virtual void dump(const Address &addr,OpCode opc,VarnodeData *outvar,VarnodeData *vars,int4 isize);
  
  std::string getPcode() const {
    return pcodeStream.str();
  }
  void clearPcode() {
    pcodeStream.str("");
    pcodeStream.clear();
  }
};

static void print_vardata(ostream &s,VarnodeData &data) {
  s << '(' << data.space->getName() << ',';
  data.space->printOffset(s,data.offset);
  s << ',' << dec << data.size << ')';
}

void PcodeRawOut::dump(const Address &addr, OpCode opc, VarnodeData *outvar, VarnodeData *vars, int4 isize) {
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

static void dumpPcode(Translate &trans) {
  PcodeRawOut emit;    // Set up the pcode dumper
  int4 length;         // Number of bytes of each machine instruction

  // MODIFY HERE THE ADDRESSES
  // -------------------------------
  Address addr(trans.getDefaultCodeSpace(), 0x80483b4); // First address to translate
  Address lastaddr(trans.getDefaultCodeSpace(), 0x80483bf); // Last address
  // -------------------------------

  std::ofstream outFile("../results/raw_pcode.txt");
    if (!outFile.is_open()) {
        std::cerr << "Failed to open output file." << std::endl;
        return;
    }

  while (addr < lastaddr) {
    //std::cout << "Processing instruction at address: " << std::hex << addr.getOffset() << std::endl;
    length = trans.oneInstruction(emit, addr); // Translate instruction
    //std::cout << "Instruction length: " << length << std::endl;

    if (length > 0) {
      addr = addr + length;                      // Advance to next instruction
      outFile << emit.getPcode();
      emit.clearPcode(); // Clear the string stream for the next instruction
    } else {
      std::cerr << "No instruction translated at address: " << std::hex << addr.getOffset() << std::endl;
      break; // Exit the loop if no instruction is translated
    }
  }

  outFile.close();
  std::cout << "Pcode dumping into output file located in '/results' completed." << std::endl;
}

// External function that is called by the main.rs program
extern "C" const char* generate_raw_pcode(const char* filename) {
    try {
        // Initialize the required Ghidra components
        ghidra::AttributeId::initialize();
        ghidra::ElementId::initialize();

        // Set up the load image using the provided filename
        MyLoadImage loader(0, filename, 0x1);

        // Set up the context object
        ghidra::ContextInternal context;

        // Set up the SLEIGH translator with the filename as the architecture type
        std::string sleighfilename = "specfiles/x86.sla"; 
        Sleigh trans(&loader, &context);

        // Read sleigh file into DOM
        DocumentStorage docstorage;
        Element *sleighroot = docstorage.openDocument(sleighfilename)->getRoot();
        docstorage.registerTag(sleighroot);
        trans.initialize(docstorage); // Initialize the translator

        // Set the default context
        context.setVariableDefault("addrsize", 1); // Address size is 32-bit
        context.setVariableDefault("opsize", 1); // Operand size is 32-bit

        dumpPcode(trans);

    } catch (const std::exception& e) {
        std::cerr << "Error in generate_raw_pcode: " << e.what() << std::endl;
        return nullptr;
    }
    return "Pcode generation completed";
  }
} // End of namespace ghidra
