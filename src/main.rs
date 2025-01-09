#![warn(useless_ptr_null_checks)]

use std::env;

pub mod high_pcode_generator;
pub mod low_pcode_generator;
pub mod pcode_generator;

fn main() {
    // Initialize the logger
    env_logger::init();
    
    let args: Vec<String> = env::args().collect();

    // We want: cargo run <filename> --[high-pcode|low-pcode] [--base-addr 0x...]
    // So let's parse them manually.
    if args.len() < 3 {
        eprintln!("Usage: cargo run <path_to_binary_file> --[high-pcode|low-pcode] [--base-addr 0x...]");
        return;
    }

    // The first arg is the executable call, so skip it
    // We'll hold user-provided values in these variables:
    let mut filename: Option<String> = None;
    let mut mode: Option<String> = None;
    let mut base_addr: u64 = 0; // Default base address

    // Start from index=1 to skip the program name
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--high-pcode" => {
                mode = Some("--high-pcode".to_string());
            }
            "--low-pcode" => {
                mode = Some("--low-pcode".to_string());
            }
            "--base-addr" => {
                // The next argument should be a hex address
                i += 1;
                if i >= args.len() {
                    eprintln!("Error: --base-addr specified but no address provided");
                    return;
                }
                let addr_str = &args[i];
                // Try to parse it in hex (with or without 0x)
                let parsed = addr_str.trim_start_matches("0x");
                match u64::from_str_radix(parsed, 16) {
                    Ok(addr_val) => {
                        base_addr = addr_val;
                    }
                    Err(e) => {
                        eprintln!("Error: could not parse base address {addr_str} due to {e}");
                        return;
                    }
                }
            }
            _ => {
                // The first unknown argument we treat as the <filename>
                if filename.is_none() {
                    filename = Some(args[i].clone());
                } else {
                    eprintln!("Unknown or extra argument: {}", args[i]);
                    eprintln!("Usage: cargo run <path_to_binary_file> --[high-pcode|low-pcode] [--base-addr 0x...]");
                    return;
                }
            }
        }
        i += 1;
    }

    // Now we need both a filename and a mode
    let filename = match filename {
        Some(f) => f,
        None => {
            eprintln!("No filename specified.");
            return;
        }
    };
    let mode = match mode {
        Some(m) => m,
        None => {
            eprintln!("No mode specified (use --high-pcode or --low-pcode).");
            return;
        }
    };

    // Dispatch
    match mode.as_str() {
        "--high-pcode" => {
            println!("Generating high pcode...");
            high_pcode_generator::generate_high_pcode(&filename);
            println!("High pcode generation completed.");
        }
        "--low-pcode" => {
            println!("Generating low pcode...");
            // Pass the base address
            let res = low_pcode_generator::generate_low_pcode(&filename, base_addr); 
            match res {
                Ok(()) => {
                    println!("Low pcode generation completed.");
                    println!("WARNING: If there were errors during P-code generation, check your output file.");
                }
                Err(e) => {
                    eprintln!("Unable to finish correctly: {e}");
                    std::process::exit(-1);
                }
            }
        }
        _ => {
            eprintln!("Invalid mode. Use --high-pcode or --low-pcode.");
        }
    }
}