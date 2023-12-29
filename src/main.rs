use std::env;
use std::io::{stdin, stdout, Write};
use std::iter::Peekable;
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::str::{Split, SplitWhitespace};

fn main() {
    loop {
        let home_dir = env::var("HOME").unwrap();
        let current_dir = env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .replace(&home_dir, "~")
            .replace("\\", "/"); // windows

        print!("{} » ", current_dir);
        let _ = stdout().flush();

        let mut input = String::new();
        let _ = stdin().read_line(&mut input);

        let mut cmds = input.trim().split(" | ").peekable();
        let mut prev_cmd = None;

        while let Some(cmd) = cmds.next() {
            let mut parts = cmd.trim().split_whitespace();
            let cmd = parts.next().unwrap();
            let args = parts;

            match cmd {
                "exit" => return,
                "cd" => cd(args, &mut prev_cmd),
                cmd => exec(args, &mut prev_cmd, &mut cmds, cmd),
            }
        }

        if let Some(mut final_cmd) = prev_cmd {
            let _ = final_cmd.wait();
        }
    }
}

fn cd<'a>(args: SplitWhitespace<'_>, prev_cmd: &'_ mut Option<Child>) {
    let new_dir = args.peekable().peek().map_or("/", |x| *x);
    let root = Path::new(new_dir);
    if let Err(e) = env::set_current_dir(&root) {
        eprintln!("{}", e);
    }

    *prev_cmd = None;
}

fn exec<'a>(
    args: SplitWhitespace<'_>,
    prev_cmd: &mut Option<Child>,
    cmds: &mut Peekable<Split<'_, &str>>,
    cmd: &str,
) {
    let stdin = prev_cmd
        .as_mut()
        .map_or(Stdio::inherit(), |output: &mut Child| {
            Stdio::from(output.stdout.take().unwrap())
        });

    let stdout = if cmds.peek().is_some() {
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
        Err(e) => {
            *prev_cmd = None;
            eprintln!("{}", e);
        }
    };
}
