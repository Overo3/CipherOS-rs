use crate::commands::{utils::Command, ListDirectory};

pub mod commands;

fn main() {
    let _cmds: Vec<Box<dyn Command>> = vec![Box::new(ListDirectory{aliases:vec![String::from("ls")]})];
    println!("{:?}",_cmds[0])
}