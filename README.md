# Pcode Generator
Tool that generates raw Pcode (--raw-pcode) and high-level Pcode (--high-pcode) from a binary file using the Ghidra API.

### Example of use
If you want to generate the raw Pcode from the binary "calculus", use the following command:
```
cargo run /tests/calculus/calculus --raw-pcode
```  
### Debug
To debug C++ part:
```
g++ -c raw_pcode_generator.cc -o raw_pcode_generator.o -I/path/to/dir/pcode-generator/ghidra/Ghidra/Includes
```
```
g++ -o raw_pcode_generator raw_pcode_generator.cc -L/path/to/dir/pcode-generator/ghidra/Ghidra/Features/Decompiler/src/decompile/cpp -ldecomp
```
