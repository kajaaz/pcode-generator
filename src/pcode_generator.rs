use io::ErrorKind::*;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::{env, fs, io};

pub fn output_file_path(input_name: &str, type_name: &str) -> io::Result<PathBuf> {
    // Find the current executable's directory
    let exe_path = env::current_exe()?;
    let exe_dir = exe_path
        .parent()
        .ok_or_else(|| io::Error::new(NotFound, "Failed to get the executable directory"))?;

    // Navigate up two levels from the executable's directory to reach the project root
    let project_root = exe_dir
        .parent()
        .and_then(|p| p.parent())
        .ok_or_else(|| io::Error::new(NotFound, "Failed to find the project root directory"))?;

    // Create the "results" directory in the project root
    let mut output_path = project_root.join("results");
    fs::create_dir_all(&output_path)?;

    // Extract the filename from the provided file path
    let file_name: &OsStr = Path::new(input_name)
        .file_name()
        .unwrap_or_else(|| OsStr::new("generated"));

    let filename: OsString = [
        file_name,
        OsStr::new("_"),
        OsStr::new(type_name),
        OsStr::new("_pcode.txt"),
    ]
    .into_iter()
    .collect();

    output_path.push(filename);

    Ok(output_path)
}

pub fn create_output_file(input_name: &str, type_name: &str) -> io::Result<fs::File> {
    let output_path = output_file_path(input_name, type_name)?;
    println!("Output file will be created at: {output_path:?}");
    fs::File::create(&output_path)
}
