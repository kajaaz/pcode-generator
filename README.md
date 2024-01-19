# Pcode Generator
Tool that generates raw Pcode and high-level Pcode from a binary file using the Ghidra API.

## Install
Make sure to build libdecomp.a in your repo before executing cargo run:
```
git clone --recursive https://github.com/kajaaz/pcode-generator.git
cd pcode-generator/ghidra/Ghidra/Features/Decompiler/src/decompile/cpp
make libdecomp.a
cd ../../../../../../..
cd src
```
Make also sure to have Rust and C++ installed.

## Usage
Getting this Pcode generator running is quite simple: 
```
USAGE:
    cargo run [ABSOLUTE PATH TO BINARY] [FLAGS]

FLAGS:
    --high-pcode                      Generate an output file with the Ghidra high level Pcode instructions
    --raw-pcode                       Generate an output file with the Ghidra raw (low level) Pcode instructions
```

## Example of use
If you want to generate the high-level Pcode of the binary "calculus", use the following command in ```pcode-generator/src```:
```
cargo run /absolute/path/to/tests/calculus/calculus --high-pcode
```  
The output file with the generated Pcode can be found in the locally created ```results``` directory at the root of the repo.

### Credits
Thanks to @niooss-ledger, @rbran and @yhql.
