#![warn(useless_ptr_null_checks)]

use std::env;
use std::ffi::CString;
use std::os::raw::c_char;
use std::any::TypeId;

pub mod high_pcode_generator;

#[link(name = "raw_pcode_generator")]
extern "C" {
    fn generate_raw_pcode(filename: *const c_char) -> *const c_char;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: cargo run <path_to_binary_file> --[high-pcode|raw-pcode]");
        return;
    }

    let filename = &args[1];
    let mode = &args[2];

    // Checking if filename is not empty
    if filename.is_empty() {
        eprintln!("Filename cannot be empty.");
        return;
    }

    match mode.as_str() {
        "--high-pcode" => {
            println!("Generating high pcode...");
            high_pcode_generator::generate_high_pcode(filename);
            println!("High pcode generation completed.");
        }
        "--raw-pcode" => {
            let c_filename = CString::new(filename.clone()).expect("CString::new failed");
            unsafe {
                println!("Generating raw pcode...");
                let result = std::panic::catch_unwind(|| generate_raw_pcode(c_filename.as_ptr()));
                match result {
                    Ok(_) => println!("Raw pcode generation completed."),
                    Err(err) => {
                        eprintln!("Error during raw pcode generation: {:?}", err);
                        
                        // Checking if the panic was caused by a C++ exception
                        if let Some(cpp_exception) = err.downcast_ref::<Box<dyn std::any::Any + Send>>() {
                        
                        if cpp_exception.type_id() == TypeId::of::<&str>() {
                            // If the C++ exception is of type &str, print it
                            if let Some(cpp_error) = cpp_exception.downcast_ref::<&str>() {
                                eprintln!("C++ Exception: {}", cpp_error);
                            } else {
                                eprintln!("Unknown C++ Exception");
                            }
                        } else {
                            
                            // Handling other types of C++ exceptions
                            eprintln!("Unknown C++ Exception Type");
                        }
                    } else {
                        eprintln!("Panic: {:?}", err);
                    }

                    }
                }
            }
        },
        _ => eprintln!("Invalid mode. Use --high-pcode or --raw-pcode."),
    }
}
