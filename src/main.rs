use std::io::BufRead;
use std::process::Command;

fn parse_command(cmd: &str) -> Result<Command, String> {
    let tokens = cmd.split_whitespace().collect::<Vec<&str>>();
    let (cmd, args) = tokens.split_first().unwrap();
    let mut command = Command::new(cmd);
    command.args(args);
    Ok(command)
}

fn parse_commands(cmd: &str) -> Result<Vec<Command>, String> {
    // Split terminator
    let raw_commands = cmd.split(';').collect::<Vec<&str>>();
    let commands = raw_commands
        .iter()
        .map(|cmd| parse_command(cmd).unwrap())
        .collect();
    Ok(commands)
}

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    loop {
        let maybe_cmd = lines.next();
        let cmd = match maybe_cmd {
            Some(Ok(cmd)) => cmd,
            Some(Err(e)) => {
                println!("Error: {}", e);
                return;
            }
            None => return,
        };

        let commands = parse_commands(&cmd).unwrap();
        for mut command in commands {
            match command.output() {
                Ok(output) => print!("{}", String::from_utf8(output.stdout).unwrap()),
                Err(e) => println!("Error: {}", e),
            }
        }
    }
}
