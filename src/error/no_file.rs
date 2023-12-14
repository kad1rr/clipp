pub fn no_file_founded() -> String {
    print!("\x1b[31m");
    println!("Error: No file found!");
    print!("\x1b[0m");

    return String::from("Error: No file found!");
}