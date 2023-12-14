use std::env;
use std::fs;
use crate::commands::{get, save, command_data, help};

mod error;
mod commands;

fn main() {
    let args: Vec<_> = env::args().collect();
    match args.len() {
        1 => display_help(),
        2 => {
            if args[1] == "--help" || args[1] == "-h" {
                display_help();
            } else {
                let data = get_clip_data(&args[1]).unwrap_or("No clip founded".parse().unwrap());
                println!("{}", data);
            }
        }
        3 if args[1] == "save" => save_clip_data(&args[2]),
        _ => {
            println!("Invalid command. Use 'clip --help' for usage information.");
            std::process::exit(1);
        }
    }
}

fn get_clip_data(filename: &str) -> Option<String> {
    get::Get::get(String::from(filename)).expect("An error occurred on get command.")
}

fn save_clip_data(filename: &str) {
    let file_content = read_file_content(filename).expect("Error reading file content.");
    save::Save::save(filename.to_string(), &file_content).expect("An error occurred on save command.");
}

fn display_help() {
    let help_data = command_data::CommandData {
        title: String::from("clipp"),
        usage: String::from(" clipp (filename)\n\tclipp save (filename)\n\tclipp [args]"),
        description: String::from("Quickly save your codes that you will use again and again."),
        long_description: String::from(
            "Clipp CLI is a tool for saving and reusing code snippets effortlessly. \
            Organize your frequently used code blocks and streamline your coding workflow.\n\n\
            Example:\n\n\
            $ clipp save my_function.py\n\
            Clip saved successfully: my_function.py"
        ),
    };

    help::Help::new(help_data);
}

fn read_file_content(file_path: &str) -> Result<String, String> {
    fs::read_to_string(file_path).map_err(|e| e.to_string())
}
