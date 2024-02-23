#![warn(useless_ptr_null_checks)]

use std::env;

pub mod high_pcode_generator;
pub mod low_pcode_generator;
pub mod pcode_generator;

fn main() {
    let args: Vec<String> = env::args().collect();
    let Some([_arg0, filename, mode]) = TryInto::<[String; 3]>::try_into(args).ok() else {
        eprintln!("Usage: cargo run <path_to_binary_file> --[high-pcode|low-pcode]");
        return;
    };

    // Checking if filename is not empty
    if filename.is_empty() {
        eprintln!("Filename cannot be empty.");
        return;
    }

    match mode.as_str() {
        "--high-pcode" => {
            println!("Generating high pcode...");
            high_pcode_generator::generate_high_pcode(&filename);
            println!("High pcode generation completed.");
        }
        "--low-pcode" => {
            println!("Generating low pcode...");
            let _ = low_pcode_generator::generate_low_pcode(&filename);
            println!("Low pcode generation completed.");
        }
        _ => eprintln!("Invalid mode. Use --high-pcode or --low-pcode."),
    }
}
