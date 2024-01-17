:construction: **REPOSITORY UNDER CONSTRUCTION, DO NOT USE IT YET** :construction:

# Pcode Generator
Tool that generates raw Pcode (--raw-pcode) and high-level Pcode (--high-pcode) from a binary file using the Ghidra API.

### Install
Make sure to build libdecomp.a in your repo before executing cargo run:
```
git clone --recursive https://github.com/kajaaz/pcode-generator.git
cd pcode-generator/ghidra/Ghidra/Features/Decompiler/src/decompile/cpp
make libdecomp.a
cd ../../../../../../..
cd src
```  
### Example of use
If you want to generate the high-level Pcode of the binary "calculus", use the following command in ```pcode-generator/src```:
```
cargo run /absolute/path/to/tests/calculus/calculus --high-pcode
```  
The output file with the generated Pcode can be found in the locally created ```results``` directory at the root of the repo.
### Debug
To debug C++ part:
```
g++ -c raw_pcode_generator.cc -o raw_pcode_generator.o -I/absolute/path/to/dir/pcode-generator/ghidra/Ghidra/Includes
```
```
g++ -o raw_pcode_generator raw_pcode_generator.cc -L/absolute/path/to/dir/pcode-generator/ghidra/Ghidra/Features/Decompiler/src/decompile/cpp -ldecomp
```
