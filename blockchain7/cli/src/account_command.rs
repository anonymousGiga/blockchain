use crate::command::Command;

pub struct AccountCommand;

impl Command for AccountCommand {
    fn get_aliases(&self) -> Vec<&'static str> {
        vec!["account", "a"]
    }

    fn execute(&self, params: &[&str]) {
        let _ = params;
        println!("execute account command!");
    }
}
