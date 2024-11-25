import pyhidra

# Start Pyhidra before importing Ghidra modules
pyhidra.start()

from ghidra.program.model.symbol import SymbolType

def analyze_with_pyhidra(binary_path, existing_symbols):
    """
    Analyze the binary using Pyhidra, specifically to get any missing symbols in the .text section.
    Avoids re-processing symbols already identified by Goblin.
    """
    with pyhidra.open_program(binary_path) as flat_api:
        program = flat_api.getCurrentProgram()
        symbol_table = program.getSymbolTable()
        image_base = program.getImageBase().getOffset()
        print(f"IMAGE_BASE:0x{image_base:x}")

        function_info = {}

        # Iterate over all symbols in Pyhidra and avoid duplicates
        for symbol in symbol_table.getAllSymbols(True):
            if symbol.getSymbolType() == SymbolType.FUNCTION:
                function_address = symbol.getAddress().getOffset()
                if function_address in existing_symbols:
                    continue  # Skip symbols already processed by goblin

                function = symbol.getObject()  # Get the function object
                function_name = symbol.getName()
                function_size = function.getBody().getNumAddresses()

                # Sanity check on function size
                if function_size == 0:
                    print(f"Warning: Function {function_name} at 0x{function_address:x} has size 0")
                elif function_size > 0x10000:
                    print(f"Warning: Large function {function_name} at 0x{function_address:x} with size {function_size}")

                # Store function address, name, and size
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
    existing_symbols = {}  # To be filled by Goblin in Rust

    results = analyze_with_pyhidra(binary_path, existing_symbols)

    # Print the results for Rust to parse
    for addr, info in results.items():
        print(f"{addr:x}\t{info['name']}\t{info['size']}")