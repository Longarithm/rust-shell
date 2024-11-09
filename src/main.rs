use std::io::{BufRead, IsTerminal, Write};
use std::process::Command;

// TODOs
// AND doesn't work because it doesn't terminate if false.
// OR doesn't work. In fact everything is just executed in a row.
// Quotes don't work. Like, "&&" is treated as &&

fn read_line(stdin: &mut dyn BufRead) -> Result<String, std::io::Error> {
    let mut line = String::new();
    stdin.read_line(&mut line)?;
    Ok(line)
}

fn parse_command(cmd: &str) -> Result<Option<Command>, std::io::Error> {
    let raw_tokens = cmd.split_whitespace().collect::<Vec<&str>>();
    if raw_tokens.is_empty() {
        return Ok(None);
    }
    let (cmd, args) = raw_tokens.split_first().unwrap();
    let mut command = Command::new(cmd);
    command.args(args);
    Ok(Some(command))
}

#[allow(unused)]
fn parse_by_or(cmd: &str) -> Result<Vec<Command>, std::io::Error> {
    let subcommands = cmd.split("||").collect::<Vec<&str>>();
    let commands = subcommands
        .iter()
        .flat_map(|subcmd| parse_command(subcmd))
        .flatten()
        .collect();
    Ok(commands)
}

fn parse_by_and(cmd: &str) -> Result<Vec<Command>, std::io::Error> {
    let subcommands = cmd.split("&&").collect::<Vec<&str>>();
    let commands = subcommands
        .iter()
        .flat_map(|subcmd| parse_command(subcmd))
        .flatten()
        .collect();
    Ok(commands)
}

fn parse_commands(cmd: &str) -> Result<Vec<Command>, std::io::Error> {
    // Split terminator
    let subcommands = cmd.split(';').collect::<Vec<&str>>();
    let commands = subcommands
        .iter()
        .flat_map(|cmd| parse_by_and(cmd).unwrap())
        .collect();
    Ok(commands)
}

fn execute_cd(command: Command) {
    let dir = command.get_args().next().unwrap();
    if let Err(e) = std::env::set_current_dir(dir) {
        eprintln!("Error: {}", e);
    }
}

fn execute_exit(command: Command) {
    let status = match command.get_args().next() {
        Some(status) => status.to_str().unwrap().parse::<i32>().unwrap(),
        None => 0,
    };
    std::process::exit(status);
}

fn main() {
    let mut stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout();

    loop {
        if stdout.is_terminal() {
            print!("> ");
        }
        stdout.flush().unwrap();

        let maybe_cmd = read_line(&mut stdin);
        let cmd = match maybe_cmd {
            Ok(cmd) => cmd,
            Err(e) => {
                eprintln!("Error: {}", e);
                return;
            }
        };

        let commands = parse_commands(&cmd).unwrap();
        for mut command in commands {
            match command.get_program().to_str().unwrap() {
                "cd" => {
                    execute_cd(command);
                }
                "exit" => {
                    return execute_exit(command);
                }
                _ => match command.output() {
                    Ok(output) => print!("{}", String::from_utf8(output.stdout).unwrap()),
                    Err(e) => eprintln!("Error: {}", e),
                },
            }
        }
    }
}
