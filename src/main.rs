use std::io::BufRead;
use std::process::Command;

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

        let tokens = cmd.split_whitespace().collect::<Vec<&str>>();
        let (cmd, args) = tokens.split_first().unwrap();

        let output = Command::new(cmd).args(args).output();

        match output {
            Ok(output) => print!("{}", String::from_utf8(output.stdout).unwrap()),
            Err(e) => println!("Error: {}", e),
        }
    }
}
