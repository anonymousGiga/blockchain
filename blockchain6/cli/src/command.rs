use std::{collections::HashMap, sync::Arc};
use crate::account_command::AccountCommand;

pub trait Command {
    fn get_aliases(&self) -> Vec<&'static str>;

    fn execute(&self, params: &[&str]) {
        let _ = params;
        println!("default, do nothing");
    }
}

pub fn parse_cmd(cmd_str: &str) -> Vec<&str> {
    cmd_str.split_ascii_whitespace().collect()
}

pub fn get_commands() -> (Vec<Arc<dyn Command>>, 
        HashMap<&'static str, Arc<dyn Command>>) {

    let commands: Vec<Arc<dyn Command>> = vec![
        Arc::new(AccountCommand {}),
    ]; 

    let mut alias_to_cmd = HashMap::new();
    for command in &commands {
        for alias in command.get_aliases() {
            alias_to_cmd.insert(alias, Arc::clone(command));
        }
    }

    (commands, alias_to_cmd)
}