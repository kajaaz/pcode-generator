# Pcode Generator
Tool that generates low-level (raw) Pcode and high-level Pcode from a binary file using the Ghidra API.

## Install
Make sure to install submodules and configure the correct path to Ghidra:
```
git clone --recursive https://github.com/kajaaz/pcode-generator.git
export GHIDRA_SRC=${HOME}/path/to/pcode-generator/ghidra
```
Make also sure to have Rust and C++ installed.

## Usage
Getting this Pcode generator running is quite simple: 
```
USAGE:
    cargo run [ABSOLUTE PATH TO BINARY] [FLAGS]

FLAGS:
    --high-pcode         Generate an output file with the Ghidra high level Pcode instructions
    --low-pcode          Generate an output file with the Ghidra low level (raw) Pcode instructions
```

Be aware that the first build will take 2 to 3 minutes. After that, the generation of the file should be done in several seconds.

You can generate the raw Pcode of a binary using Pcode-generator and then use [Pcode-parser](https://github.com/kajaaz/pcode-parser/tree/main) to parse the produced pcode. 

## Example of use
If you want to generate the high-level Pcode of the binary "calculus", use the following command in ```pcode-generator/src```:
```
cargo run /absolute/path/to/tests/calculus/calculus --high-pcode
```  
The output file with the generated Pcode can be found in the locally created ```results``` directory at the root of the repo.

### Credits
Thanks to @rbran, @niooss-ledger and @yhql.
