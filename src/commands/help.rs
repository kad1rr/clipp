
use crate::commands::command_data::CommandData;

pub struct Help {}
impl Help {

    pub fn new(command_data: CommandData) -> bool {
        let header = format!("{} - {}", command_data.title, command_data.description);
        let usage = format!("Usage: {}", command_data.usage);
        let description = command_data.long_description;
        println!("{}\n", header);
        println!("{}\n", usage);
        println!("{}", description);
        return true;
    }
}