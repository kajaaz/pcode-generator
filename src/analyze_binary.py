import pyhidra

# Start Pyhidra
pyhidra.start()

from ghidra.program.model.symbol import SymbolType
from ghidra.program.flatapi import FlatProgramAPI

def analyze_binary(binary_path):
    with pyhidra.open_program(binary_path) as flat_api:
        program = flat_api.getCurrentProgram()
        symbol_table = program.getSymbolTable()
        
        function_info = {}

        # Iterate over all symbols and look for functions
        for symbol in symbol_table.getAllSymbols(True):
            if symbol.getSymbolType() == SymbolType.FUNCTION:
                function = symbol.getObject()  # Get the function object
                function_name = symbol.getName()
                function_size = function.getBody().getNumAddresses()
                function_address = symbol.getAddress().getOffset()

                # Sanity check on function size
                if function_size == 0:
                    print(f"Warning: Function {function_name} at 0x{function_address:x} has size 0")
                elif function_size > 0x10000:  # Threshold for unusually large functions
                    print(f"Warning: Large function {function_name} at 0x{function_address:x} with size {function_size}")

                # Store the function address, name, and size
                function_info[function_address] = {
                    'name': function_name,
                    'size': function_size
                }

        return function_info

if __name__ == "__main__":
    import sys
    if len(sys.argv) != 2:
        print("Usage: analyze_binary.py <binary_path>")
        sys.exit(1)

    binary_path = sys.argv[1]
    results = analyze_binary(binary_path)
    
    # Print the results in a structured format for Rust to parse
    for addr, info in results.items():
        print(f"{addr:x}:{info['name']}:{info['size']}")
