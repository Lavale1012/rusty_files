mod test_banner;

use std::fs;
use std::fs::File;

use std::io::Read;
use std::io::Write;

fn main() -> std::io::Result<()> {
    test_banner::banner();
    println!("----- Welcome to rusty file manager ------");

    loop {
        let current_dir_promt = std::env::current_dir().unwrap();
        print!("{} -> ", current_dir_promt.display());
        std::io::stdout().flush().unwrap(); // Flush so prompt shows up

        let mut input = String::new();

        std::io::stdin().read_line(&mut input).unwrap();

        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "lf" => {
                let dir = if parts.len() > 1 { parts[1] } else { "." };

                let current_dir = std::env::current_dir().unwrap();
                if dir.is_empty() {
                    println!("Please provide a directory name.");
                    continue;
                }

                if parts.len() > 2 && parts[2] == "-filter" {
                    if parts.len() < 4 {
                        println!("Please provide a filter extension (e.g. rs, txt).");
                        continue;
                    }
                    let filter = parts[3];
                    let paths: Vec<_> = match fs::read_dir(dir) {
                        Ok(read_dir) => read_dir
                            .filter_map(Result::ok)
                            .map(|entry| entry.path())
                            .collect(),
                        Err(_) => Vec::new(),
                    };
                    let filtered_files: Vec<_> = paths
                        .iter()
                        .filter(|entry| entry.extension().map_or(false, |ext| ext == filter))
                        .collect();

                    println!("                                          ");
                    println!("----- Files in the current directory -----");
                    println!("                                          ");
                    println!("{}", current_dir.display());
                    println!("------------------------------------------");
                    for file in filtered_files {
                        let file_name = file.file_name().unwrap().to_string_lossy();
                        println!("{}", file_name);
                    }
                } else {
                    match std::fs::read_dir(dir) {
                        Ok(entries) => {
                            println!("                                          ");
                            println!("----- Files in the current directory -----");
                            println!("                                          ");
                            println!("{}", current_dir.display());
                            println!("------------------------------------------");
                            for entry in entries {
                                let entry = entry.unwrap();
                                let file_name = entry.file_name();
                                println!("{}", file_name.to_string_lossy());
                            }
                        }
                        Err(e) => println!("Error reading directory: {}", e),
                    }
                }
            }
            "slf" => {
                if parts.len() >= 4 && parts[1] == "-ren" {
                    let selected_file = parts[2];
                    let renamed_file = parts[3];
                    match std::fs::rename(selected_file, renamed_file) {
                        Ok(_) => println!("File renamed successfully!"),
                        Err(e) => println!("Error renaming file: {}", e),
                    }
                } else {
                    let file_name = if parts.len() > 1 { parts[1].trim() } else { "" };

                    if file_name.is_empty() {
                        println!("Please provide a file name.");
                        continue;
                    }

                    let mut content = String::new();
                    match File::open(file_name) {
                        Ok(mut file) => {
                            if let Err(e) = file.read_to_string(&mut content) {
                                println!("Error reading file content: {}", e);
                                continue;
                            }

                            println!("----------- File content -----------------");
                            println!("               {}                       ", file_name);
                            println!("------------------------------------------");
                            println!("{}", content);
                            println!("------------------------------------------\n");
                        }
                        Err(e) => println!("Failed to open file: {}", e),
                    }
                }
            }
            "sldir" => {
                if parts.len() > 1 {
                    let dir_name = parts[1];
                    std::env::set_current_dir(dir_name).unwrap();
                }
            }
            "cf" => {
                let file_name = if parts.len() > 1 { parts[1].trim() } else { "" };
                if file_name.is_empty() {
                    println!("Please provide a file name.");
                    continue;
                }
                let _file = match File::create(file_name) {
                    Ok(_file) => _file,
                    Err(e) => {
                        println!("Error creating file: {}", e);
                        continue;
                    }
                };
            }
            "df" => {
                let file_name = if parts.len() > 1 { parts[1].trim() } else { "" };
                if file_name.is_empty() {
                    println!("Please provide a file name.");
                    continue;
                }
                match std::fs::remove_file(file_name) {
                    Ok(_) => println!("File deleted successfully!"),
                    Err(e) => println!("Error deleting file: {}", e),
                }
            }
            "cdir" => {
                let dir_name = if parts.len() > 1 { parts[1].trim() } else { "" };
                if dir_name.is_empty() {
                    println!("Please provide a directory name.");
                    continue;
                }
                let _dir = match fs::create_dir(dir_name) {
                    Ok(_dir) => _dir,
                    Err(e) => {
                        println!("Error creating directory: {}", e);
                        continue;
                    }
                };
            }
            "ddir" => {
                let dir_name = if parts.len() > 1 { parts[1].trim() } else { "" };
                if dir_name.is_empty() {
                    println!("Please provide a directory name.");
                    continue;
                }
                match fs::remove_dir(dir_name) {
                    Ok(_) => println!("Directory deleted successfully!"),
                    Err(e) => println!("Error deleting directory: {}", e),
                }
            }
            "help" => {
                println!("Available commands:");
                println!("lf <directory>: Show files in a specific directory");
                println!(
                    "lf <directory> -filter <extension>: Show files with a specific extension"
                );
                println!("slf <file_name>: Open a file");
                println!("slf -ren <old_name> <new_name>: Rename a file");
                println!("slf <file>: Open a file");
                println!("cf <file name>: Create a file");
                println!("cdir <directory>: Create a directory");
                println!("ddir <directory>: Delete a directory");
                println!("df <file>: Delete a file");
                println!("sldir <directory>: Change directory");
                println!("help: Show this help message");
                println!("quit: Exit the program");
            }

            "quit" => {
                println!("Exiting...");
                return Ok(());
            }
            _ => {
                println!("Invalid option! Please try again.");
            }
        }
    }
}
