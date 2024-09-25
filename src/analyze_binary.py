import pyhidra

# Start Pyhidra
pyhidra.start()

# Now import Ghidra-related modules after Pyhidra is initialized
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
