# Pcode Generator
Tool that generates low-level (raw) Pcode and high-level Pcode from a binary file using the Ghidra API.

Currently covered and partiely tested binaries handling from following languages : Golang, C.

## Install
Make sure to have Rust and C++ installed.

Install submodules and the repo with the ```--recursive``` flag:
```
sudo apt-get update
sudo apt install binutils-dev bison
git clone --recursive https://github.com/kajaaz/pcode-generator.git
```
You will need to adapt the path to your ```include/c++``` inside the ```ghidra-decompiler/build.rs``` file:
```
.clang_arg("-I/usr/include/c++/11")
```
If needed, export the correct path to Ghidra:
```
export GHIDRA_SRC=${HOME}/path/to/pcode-generator/ghidra
export GHIDRA_INSTALL_DIR=/path/to/ghidra
```

## Usage
If you work with Go binaries, make sure to have built them with BoringSSL:
```
GOEXPERIMENT=boringcrypto go build .
```
Getting this Pcode generator running is quite simple: 
```
USAGE:
    cargo run [ABSOLUTE PATH TO BINARY] [FLAGS] [OPTION]

FLAGS:
    --high-pcode         Generate an output file with the Ghidra high level Pcode instructions
    --low-pcode          Generate an output file with the Ghidra low level (raw) Pcode instructions

OPTION:
    --base-addr          Define the base address only for low P-Code generation (default at 0)
```

Be aware that the first build will take 2 to 3 minutes. After that, the generation of the file should be done in several seconds.

You can generate the raw Pcode of a binary using Pcode-generator and then use [Pcode-parser](https://github.com/kajaaz/pcode-parser/tree/main) to parse the produced pcode. 

## Example of use
If you want to generate the high-level Pcode of the binary "calculus", use the following command in ```pcode-generator/src```:
```
cargo run /absolute/path/to/tests/calculus/calculus --low-pcode --base-addr 0x200000
```  
The output file with the generated Pcode can be found in the locally created ```results``` directory at the root of the repo.

### Credits
Thanks to @rbran, @niooss-ledger and @yhql.
