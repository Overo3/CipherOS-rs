use std::{collections::HashMap, rc::Rc};

#[derive(Debug)]
pub enum ExecutionError {
    IoError(std::io::Error),
    StringError(String),
}

impl From<std::io::Error> for ExecutionError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

pub trait Command {
    fn execute(&self, args: &Vec::<String>) -> Result<String, ExecutionError>;
    fn get_aliases(&self) -> Result<&Vec<String>, String>;
    fn debug(&self) -> &Vec<String>;
}

impl std::fmt::Debug for dyn Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut alias_string: String = String::new();
        for alias in self.get_aliases().unwrap() {
            alias_string.push_str(alias);
        }
        write!(f, "Command [aliases: {}]", alias_string)
    }
}

pub fn register_command(command_array: &mut HashMap<String, Rc<Box<dyn Command>>>, command: Box<dyn Command>) -> Result<(), String> {
    let ref_to_command = Rc::new(command);
    for alias in ref_to_command.get_aliases().unwrap() {
        command_array.insert(alias.clone(), ref_to_command.clone());
    }
    Ok(())
}