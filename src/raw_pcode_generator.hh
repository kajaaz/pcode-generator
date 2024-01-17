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
/// \file raw_pcode_generator.hh
/// \brief Classes and API for accessing a binary load image

#ifndef __RAW_PCODE_GENERATOR_HH__
#define __RAW_PCODE_GENERATOR_HH__

#include <fstream>
#include <cstring>
#include <exception>
#include <string>
#include "../ghidra/Ghidra/Features/Decompiler/src/decompile/cpp/address.hh"
#include "../ghidra/Ghidra/Features/Decompiler/src/decompile/cpp/loadimage.hh"
#include "../ghidra/Ghidra/Features/Decompiler/src/decompile/cpp/pcoderaw.hh"
#include "../ghidra/Ghidra/Features/Decompiler/src/decompile/cpp/globalcontext.hh"
#include "../ghidra/Ghidra/Features/Decompiler/src/decompile/cpp/sleigh.hh"
#include "../ghidra/Ghidra/Features/Decompiler/src/decompile/cpp/semantics.hh"
#include "../ghidra/Ghidra/Features/Decompiler/src/decompile/cpp/translate.hh"
#include "../ghidra/Ghidra/Features/Decompiler/src/decompile/cpp/opbehavior.hh"
#include "../ghidra/Ghidra/Features/Decompiler/src/decompile/cpp/opcodes.hh"

namespace ghidra {

// Forward declarations
class Sleigh;
class ContextInternal;

// Extend AssemblyEmit
class AssemblyEmitExtend : public AssemblyEmit {
public:
    void dump(const Address &addr, const std::string &mnem, const std::string &body) override;
};

// Extend PcodeEmit
class PcodeEmitExtend : public PcodeEmit {
    std::vector<PcodeOpRaw> pcodeOps;
    std::ofstream outputFile;

public:
    ~PcodeEmitExtend();
    void dump(const Address &addr, OpCode opc, VarnodeData *outvar, VarnodeData *vars, int4 isize) override;
    const std::vector<PcodeOpRaw>& getPcodeOps() const;
};

bool isUnaryOperation(OpCode opc);

struct RawLoadImagePcodeFunc {
    Address address;    ///< Start of function
    std::string name;   ///< Name of function
};

struct RawLoadImagePcodeSection {
    Address address;    ///< Starting address of section
    uintb size;         ///< Number of bytes in section
    uint4 flags;        ///< Properties of the section
};
class RawLoadImagePcode : public LoadImage {
    uintb vma;          ///< Address of the first byte in the file
    std::ifstream *thefile;  ///< Main file stream for image
    uintb filesize;     ///< Total number of bytes in the load image/file
    AddrSpace *spaceid;     ///< Address space that the file bytes are mapped to
public:
    RawLoadImagePcode(const std::string &f); ///< RawLoadImagePcode constructor
    void attachToSpace(AddrSpace *id) { spaceid = id; }  ///< Attach the raw image to a particular space
    void open(void);                   ///< Open the raw file for reading
    virtual ~RawLoadImagePcode(void);             ///< RawLoadImagePcode destructor

    // Override the virtual functions
    virtual void loadFill(uint1 *ptr, int4 size, const Address &addr) override;
    virtual std::string getArchType(void) const override;
    virtual void adjustVma(long adjust) override;
    virtual uintb getFileSize(void) const; // New function added here, not exiting in LoadImage
};

} // End namespace ghidra
#endif
