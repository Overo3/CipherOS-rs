use std::{collections::HashMap, rc::Rc, env};

use crate::commands::{utils::{Command, register_command}, ListDirectory};

pub mod commands;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut command_array: HashMap<String, Rc<Box<dyn Command>>> = Default::default();
    let mut cmds: Vec<Box<dyn Command>> = vec![Box::new(ListDirectory{aliases:vec![String::from("ls")]})];
    register_command(&mut command_array, cmds.pop().unwrap()).unwrap();
    println!("{}",command_array["ls"].execute(&args).unwrap());
}