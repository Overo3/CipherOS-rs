pub mod utils;

use crate::commands::utils::Command;

pub struct ListDirectory {
    pub aliases: Vec<String>
}

impl ListDirectory {
}

impl Command for ListDirectory {
    fn execute(&self, _args: &Vec<String>) -> Result<String, String> {
        Ok("".to_string())
    }
    fn get_aliases(&self) -> Result<&Vec<String>, String> {
        Ok(&self.aliases)
    }
    fn debug(&self) -> &Vec<String> {
        &self.aliases
    }
}