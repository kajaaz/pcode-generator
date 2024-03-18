use std::collections::{BTreeSet, HashMap};
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use regex::Regex;
use xml::reader::{EventReader, XmlEvent};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run <INPUT_PCODE_PATH>");
        std::process::exit(1);
    }
    
    let input_path = &args[1];
    let output_path = "output.txt";
    
    // Automatically construct the path to callother-database.txt
    let mut callother_database_path = env::current_dir()?;
    callother_database_path.push("src");
    callother_database_path.push("callother-database.txt");

    let input_file = File::open(input_path)?;
    let reader = BufReader::new(input_file);
    let mut output_file = File::create(output_path)?;

    let callother_db = load_callother_database(callother_database_path.to_str().unwrap())?;

    let call_other_regex = Regex::new(r"CALLOTHER \(const,0x([0-9a-fA-F]+),\d+\)").unwrap();
    let mut operations_used = BTreeSet::new();

    for line in reader.lines().filter_map(Result::ok) {
        if let Some(caps) = call_other_regex.captures(&line) {
            let index_hex = caps.get(1).unwrap().as_str();
            let index = u32::from_str_radix(index_hex, 16).unwrap_or_default();

            if let Some(op_name) = callother_db.get(&index) {
                operations_used.insert((index, op_name.clone()));
            }
        }
    }

    // Output the unique and sorted operations used
    writeln!(output_file, "Used CALLOTHER operations:")?;
    for (index, name) in &operations_used {
        writeln!(output_file, "Index: 0x{:x}, Name: {}", index, name)?;
    }

    Ok(())
}

fn load_callother_database(path: &str) -> io::Result<HashMap<u32, String>> {
    let file = File::open(path)?;
    let file_reader = BufReader::new(file);
    let parser = EventReader::new(file_reader);

    let mut db = HashMap::new();

    let mut current_name = String::new();
    let mut current_index: Option<u32> = None;

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) if name.local_name == "userop" => {
                for attr in attributes {
                    match attr.name.local_name.as_str() {
                        "name" => current_name = attr.value,
                        "index" => current_index = Some(u32::from_str_radix(&attr.value, 10).unwrap_or_default()),
                        _ => {}
                    }
                }
            }
            Ok(XmlEvent::EndElement { name }) if name.local_name == "userop" => {
                if let Some(index) = current_index.take() {
                    db.insert(index, current_name.clone());
                }
            }
            Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
            _ => {}
        }
    }

    Ok(db)
}
