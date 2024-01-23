use io::ErrorKind::*;
use std::ffi::{OsStr, OsString};
use std::path::Path;
use std::{env, fs, io};

pub fn create_output_file(input_name: &str, type_name: &str) -> io::Result<fs::File> {
    // New : Find the current executable's directory
    let exe_path = env::current_exe()?;
    let exe_dir = exe_path
        .parent()
        .ok_or_else(|| io::Error::new(NotFound, "Failed to get the executable directory"))?;

    // New : Navigate up two levels from the executable's directory to reach the project root
    let project_root = exe_dir
        .parent()
        .and_then(|p| p.parent())
        .ok_or_else(|| io::Error::new(NotFound, "Failed to find the project root directory"))?;

    // New : Create the "results" directory in the project root
    let mut output_path = project_root.join("results");
    fs::create_dir_all(&output_path)?;

    // New : Extract the filename from the provided file path
    let file_stem: &OsStr = Path::new(input_name)
        .file_stem()
        .unwrap_or_else(|| OsStr::new("generated"));

    let filename: OsString = [
        file_stem,
        OsStr::new("_"),
        OsStr::new(type_name),
        OsStr::new("_pcode.txt"),
    ]
    .into_iter()
    .collect();

    // New : Modify the output filename to be inside the "results" directory
    output_path.push(filename);

    // New : Print the full path of the output file for verification
    println!("Output file will be created at: {output_path:?}");

    fs::File::create(&output_path)
}
