use crate::commands::utils::Command;

pub struct ThisCommand {
}

impl Command for ThisCommand {
    fn execute(&self, _args: &Vec<String>) -> Result<String, String> {
        Ok("".to_string())
    }
}