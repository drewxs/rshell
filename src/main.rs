use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};

use colored::Colorize;

fn main() {
    loop {
        let username = env::var("USER").unwrap_or("unknown".to_string());
        let home_dir = env::var("HOME").unwrap_or("/".to_string());
        let current_dir = env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .replace(&home_dir, "~")
            .replace("\\", "/"); // windows

        print!(
            "{} {} {} » ",
            username.cyan(),
            "::".red(),
            current_dir.cyan()
        );
        let _ = io::stdout().flush().unwrap();

        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);

        let mut cmds = input.trim().split(" | ").peekable();
        let mut prev_cmd = None;

        while let Some(cmd) = cmds.next() {
            let mut parts = cmd.trim().split_whitespace();
            let cmd = parts.next().unwrap();
            let args: Vec<&str> = parts.collect();

            match cmd {
                "exit" => return,
                "cd" => cd(args, &mut prev_cmd),
                cmd => exec(args, &mut prev_cmd, cmd, cmds.peek().is_some()),
            }
        }

        if let Some(mut final_cmd) = prev_cmd {
            let _ = final_cmd.wait();
        }
    }
}

fn cd(args: Vec<&str>, prev_cmd: &mut Option<Child>) {
    match args.get(0) {
        Some(&path) => {
            if let Err(error) = env::set_current_dir(Path::new(path)) {
                eprintln!("{}", error);
            }
        }
        None => {
            if let Ok(home_dir) = env::var("HOME") {
                if let Err(error) = env::set_current_dir(Path::new(&home_dir)) {
                    eprintln!("{}", error);
                }
            }
        }
    };
    *prev_cmd = None;
}

fn exec(args: Vec<&str>, prev_cmd: &mut Option<Child>, cmd: &str, has_next_cmd: bool) {
    let stdin = match prev_cmd.as_mut() {
        Some(output) => Stdio::from(output.stdout.take().unwrap()),
        None => Stdio::inherit(),
    };

    let stdout = if has_next_cmd {
        Stdio::piped()
    } else {
        Stdio::inherit()
    };

    let output = Command::new(cmd)
        .args(args)
        .stdin(stdin)
        .stdout(stdout)
        .spawn();

    match output {
        Ok(output) => {
            *prev_cmd = Some(output);
        }
        Err(error) => {
            *prev_cmd = None;
            eprintln!("{}", error);
        }
    };
}
