use std::io::{BufRead, IsTerminal, Write};
use std::process::Command;

fn read_line(stdin: &mut dyn BufRead) -> Result<String, std::io::Error> {
    let mut line = String::new();
    stdin.read_line(&mut line)?;
    Ok(line)
}

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
    let mut stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout();
    loop {
        if stdout.is_terminal() {
            print!("{}", "> ");
            stdout.flush().unwrap();
        }

        let maybe_cmd = read_line(&mut stdin);
        let cmd = match maybe_cmd {
            Ok(cmd) => cmd,
            Err(e) => {
                println!("Error: {}", e);
                return;
            }
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
