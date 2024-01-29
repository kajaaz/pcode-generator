# Pcode Generator
Tool that generates raw Pcode and high-level Pcode from a binary file using the Ghidra API.

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
    --raw-pcode          Generate an output file with the Ghidra raw (low level) Pcode instructions
```
You need to modify the base and end addresses in ```/src/low_pcode_generator.rs``` to define the range of code that you want to translate to Pcode.

```
let base_addr = 0x100;  // Your base address
let end_addr = 0x200;   // Your end address
```  

Be also aware that the first build will take 2 to 3 minutes. After that, the generation of the file should be done in several seconds.
## Example of use
If you want to generate the high-level Pcode of the binary "calculus", use the following command in ```pcode-generator/src```:
```
cargo run /absolute/path/to/tests/calculus/calculus --high-pcode
```  
The output file with the generated Pcode can be found in the locally created ```results``` directory at the root of the repo.

### Credits
Thanks to @rbran, @niooss-ledger and @yhql.
