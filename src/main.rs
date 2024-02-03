#![warn(useless_ptr_null_checks)]

use std::env;
use std::io::Write;

pub mod high_pcode_generator;
pub mod low_pcode_generator;

fn write_file(output_file: &str, generator: impl Iterator<Item = String>) {
    let mut out_file = std::fs::File::create(output_file).unwrap();
    for pcode in generator {
        out_file.write_all(pcode.as_bytes()).unwrap();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let Some([_arg0, in_file, out_file, mode]) = TryInto::<[String; 4]>::try_into(args).ok() else {
        eprintln!(
            "Usage: cargo run <path_to_binary_file> <path_to_output_file> --[high-pcode|raw-pcode]"
        );
        return;
    };

    // Checking if filename is not empty
    if in_file.is_empty() {
        eprintln!("Filename cannot be empty.");
        return;
    }

    match mode.as_str() {
        "--high-pcode" => {
            println!("Generating high pcode...");
            write_file(
                &out_file,
                high_pcode_generator::generate_high_pcode(&in_file).unwrap(),
            );
            println!("High pcode generation completed.");
        }
        "--raw-pcode" => {
            println!("Generating low pcode...");
            write_file(
                &out_file,
                low_pcode_generator::generate_low_pcode(&in_file).unwrap(),
            );
            println!("Low pcode generation completed.");
        }
        _ => eprintln!("Invalid mode. Use --high-pcode or --raw-pcode."),
    }
}
