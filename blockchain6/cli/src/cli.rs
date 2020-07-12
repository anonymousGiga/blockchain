use rustyline::error::ReadlineError;
use rustyline::Editor;
use crate::command;

pub struct Cli;

impl Cli {
    pub fn start() {
        let mut r1 = Editor::<()>::new();
        if r1.load_history("command_history.txt").is_err() {
            println!("No previous history.");
        }

        let (_commands, alias_to_cmd) = command::get_commands();

        loop {
            let readline = r1.readline(">>");
            match readline {
                Ok(line) => {
                    let params = command::parse_cmd(&line);
                    if params.is_empty() {
                        continue;
                    }

                    match alias_to_cmd.get(&params[0]) {
                        Some(cmd) => {
                            cmd.execute(&params);
                        }
                        None => match params[0] {
                            "quit" | "q!" => break,
                            "help" | "h" => println!("Print help command"),
                            "" => continue,
                            x => println!("Unknown command: {:?}", x),
                        },
                    }

                    r1.add_history_entry(line.as_str());
                    println!("Line: {}", line);
                },
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break
                },
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break
                },
                Err(err) => {
                    println!("Error: {:?}", err);
                    break
                }
            }
        }

        r1.save_history("history.txt").unwrap();
    }
}

