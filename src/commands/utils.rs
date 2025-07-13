use std::{collections::HashMap, rc::Rc};

pub trait Command {
    fn execute(&self, args: &Vec::<String>) -> Result<String, String>;
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

pub fn register_command(mut command_array: HashMap<String, Rc<Box<dyn Command>>>, command: Box<dyn Command>) -> HashMap<String, Rc<Box<dyn Command>>> {
    let ref_to_command = Rc::new(command);
    for alias in ref_to_command.get_aliases().unwrap() {
        command_array.insert(alias.clone(), ref_to_command.clone()).unwrap();
    }
    command_array
}