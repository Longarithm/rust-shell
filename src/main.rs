use std::io::{BufRead, IsTerminal, Write};
use std::process::Command;

fn read_line(stdin: &mut dyn BufRead) -> Result<String, std::io::Error> {
    let mut line = String::new();
    stdin.read_line(&mut line)?;
    Ok(line)
}

// enum Token<'a> {
//     And,
//     Or,
//     Text(&'a str),
// }

// fn parse_token(token: &str) -> Result<Token, std::io::Error> {
//     match token {
//         "&&" => Ok(Token::And),
//         "||" => Ok(Token::Or),
//         _ => Ok(Token::Text(token)),
//     }
// }

// fn parse_separated(tokens: Vec<Token>) -> Result<Vec<Command>, std::io::Error> {
//     let mut commands = Vec::new();

//     for token in tokens {
//         match token {
//             Token::Text(text) => {
//                 commands.push(command);
//             }
//             _ => {}
//         }
//     }
//     Ok(commands)
// }

// let raw_tokens = cmd.split_whitespace().collect::<Vec<&str>>();
// let (cmd, args) = token.split_once(' ').unwrap();
// let mut command = Command::new(cmd);
// command.args(args);

fn parse_command(cmd: &str) -> Result<Command, std::io::Error> {
    let raw_tokens = cmd.split_whitespace().collect::<Vec<&str>>();
    let (cmd, args) = raw_tokens.split_first().unwrap();
    let mut command = Command::new(cmd);
    command.args(args);
    Ok(command)
}

fn parse_by_ampersands(cmd: &str) -> Result<Vec<Command>, std::io::Error> {
    let subcommands = cmd.split("&&").collect::<Vec<&str>>();
    let commands = subcommands
        .iter()
        .map(|subcmd| parse_command(subcmd).unwrap())
        .collect();
    Ok(commands)
}

fn parse_commands(cmd: &str) -> Result<Vec<Command>, std::io::Error> {
    // Split terminator
    let subcommands = cmd.split(';').collect::<Vec<&str>>();
    let commands = subcommands
        .iter()
        .map(|cmd| parse_by_ampersands(cmd).unwrap())
        .flatten()
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
