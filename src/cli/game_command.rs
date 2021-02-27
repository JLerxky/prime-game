use super::command::Command;

pub struct GameCommand;

impl Command for GameCommand {
    fn get_aliases(&self) -> Vec<&'static str> {
        vec!["account", "a"]
    }

    fn execute(&self, params: &[&str]) {
        let _ = params;
        println!("execute game command!");
    }
}
