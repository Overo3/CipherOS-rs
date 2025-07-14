pub mod utils;

use crate::commands::utils::{Command, ExecutionError};
use std::{fs, io};

pub struct ListDirectory {
    pub aliases: Vec<String>
}

impl ListDirectory {
}

impl Command for ListDirectory {
    fn execute(&self, args: &Vec<String>) -> Result<String, ExecutionError> {
        let mut listing: String = Default::default();
        let path = &args[1];
        let entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
        for entry in entries {
            listing.push_str(entry.to_str().ok_or(ExecutionError::StringError("Invalid path.".to_string()))?);
            listing.push_str("\n");
        }
        return Ok(listing)   
    }

    fn get_aliases(&self) -> Result<&Vec<String>, String> {
        Ok(&self.aliases)
    }

    fn debug(&self) -> &Vec<String> {
        &self.aliases
    }
}