use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: copy_file <source_file> <destination_path>");
        return Ok(());
    }

    // Parse source and destination paths
    let source_path = Path::new(&args[1]);
    let destination_path = Path::new(&args[2]);

    // get source filename
    let file_name = source_path.file_name().unwrap();

    // create target folder
    fs::create_dir_all(destination_path)?;

    // Concatenate target file paths
    let mut destination_file_path = PathBuf::new();
    destination_file_path.push(destination_path);
    destination_file_path.push(file_name);

    // copy files
    fs::copy(source_path, destination_file_path)?;

    println!("File copied successfully!");

    Ok(())
}
