import re

# Define the base address constants
base_address = 0x100000
offset_address = 0x7ffff7ffd000

# Function to calculate the new address
def calculate_new_address(address):
    return address - base_address + offset_address

# Read the file
with open('/home/kgorna/Documents/tools/pcode-generator/results/0x7ffff7ffd000-0x7ffff7fff000_low_pcode.txt', 'r') as file:
    content = file.read()

# Regex to find addresses that are alone on a line
address_pattern = r'^\s*(0x[0-9a-fA-F]+)\s*$'

# Function to replace addresses at the beginning of a line
def replace_addresses(match):
    original_address = int(match.group(1), 16)  # Convert hex string to integer
    new_address = calculate_new_address(original_address)  # Perform the calculation
    return f'0x{new_address:x}'  # Return the new address in hexadecimal

# Replace only the addresses that appear alone on a line
new_content = re.sub(address_pattern, replace_addresses, content, flags=re.MULTILINE)

# Write the modified content to a new file
with open('modified_file.txt', 'w') as new_file:
    new_file.write(new_content)

print("Addresses have been replaced and saved to 'modified_file.txt'.")
