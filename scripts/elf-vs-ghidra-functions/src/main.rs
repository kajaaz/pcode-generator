use std::fs::File;
use std::io::{self, BufRead};
use std::collections::BTreeMap;
use xml::reader::{EventReader, XmlEvent};

// Function to parse the text file and extract function addresses and sizes
fn parse_text_file(filename: &str) -> io::Result<BTreeMap<u64, u64>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut functions = BTreeMap::new();

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("0x") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 2 {
                let addr = u64::from_str_radix(&parts[0][2..], 16).unwrap();
                let size = parts[1].parse::<u64>().unwrap();
                functions.insert(addr, size);
            }
        }
    }

    Ok(functions)
}

// Function to parse the XML file and extract function addresses and sizes
fn parse_xml_file(filename: &str) -> io::Result<BTreeMap<u64, u64>> {
    let file = File::open(filename)?;
    let parser = EventReader::new(file);
    let mut functions = BTreeMap::new();
    let mut inside_functions_section = false;
    let mut entry_point: Option<u64> = None;
    let mut function_size: Option<u64> = None;

    for event in parser {
        match event {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                if name.local_name == "FUNCTIONS" {
                    inside_functions_section = true;
                } else if inside_functions_section && name.local_name == "FUNCTION" {
                    // Extract ENTRY_POINT attribute
                    for attr in attributes {
                        if attr.name.local_name == "ENTRY_POINT" {
                            entry_point = Some(u64::from_str_radix(&attr.value[2..], 16).unwrap());
                        }
                    }
                } else if inside_functions_section && name.local_name == "ADDRESS_RANGE" {
                    let mut start_addr = None;
                    let mut end_addr = None;
                    // Extract START and END attributes
                    for attr in attributes {
                        if attr.name.local_name == "START" {
                            start_addr = Some(u64::from_str_radix(&attr.value[2..], 16).unwrap());
                        } else if attr.name.local_name == "END" {
                            end_addr = Some(u64::from_str_radix(&attr.value[2..], 16).unwrap());
                        }
                    }
                    // Calculate size
                    if let (Some(start), Some(end)) = (start_addr, end_addr) {
                        function_size = Some(end - start);
                    }
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                if name.local_name == "FUNCTIONS" {
                    inside_functions_section = false;
                } else if inside_functions_section && name.local_name == "FUNCTION" {
                    // Insert the parsed function address and size into the map
                    if let (Some(addr), Some(size)) = (entry_point, function_size) {
                        functions.insert(addr, size);
                    }
                    entry_point = None;
                    function_size = None;
                }
            }
            _ => {}
        }
    }

    Ok(functions)
}

// Function to compare the function sizes between two maps with a tolerance for size difference of 1
fn compare_functions(
    text_functions: &BTreeMap<u64, u64>,
    xml_functions: &BTreeMap<u64, u64>,
) {
    let mut mismatches = Vec::new(); // Vector to store mismatches

    for (addr, text_size) in text_functions {
        if let Some(xml_size) = xml_functions.get(addr) {
            let result = if (text_size == xml_size) || ((*text_size as i64 -* xml_size as i64).abs() == 1) {
                "yes"
            } else {
                mismatches.push((addr, *text_size, *xml_size));
                "no"
            };
            println!("Address: 0x{:x}, ELF size: {}, XML size: {}, Match: {}", addr, text_size, xml_size, result);
        } else {
            println!("Address: 0x{:x}, ELF size: {}, XML size: Not Found", addr, text_size);
        }
    }

    // Print summary of mismatches
    if !mismatches.is_empty() {
        println!("\nSummary of mismatches:");
        for (addr, text_size, xml_size) in mismatches {
            println!("Address: 0x{:x}, ELF size: {}, XML size: {}, Match: no", addr, text_size, xml_size);
        }
    } else {
        println!("\nAll functions match!");
    }
}

fn main() -> io::Result<()> {
    let text_file = "/home/kgorna/Documents/tools/pcode-generator/scripts/elf-vs-ghidra-functions/additiongo-taskset1_functions.txt";
    let xml_file = "/home/kgorna/Documents/tools/pcode-generator/scripts/elf-vs-ghidra-functions/additiongo-taskset1.xml";

    let text_functions = parse_text_file(text_file)?;
    let xml_functions = parse_xml_file(xml_file)?;

    // Compare the functions from both files
    compare_functions(&text_functions, &xml_functions);

    Ok(())
}

