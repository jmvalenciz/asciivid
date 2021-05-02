use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

pub struct Cmd{}

impl Cmd{
    pub fn execute(command: &str, args: Vec<&str>){
        let stdout = Command::new(command)
            .args(args)
            .stdout(Stdio::piped())
            .spawn()
            .expect("Unable to spawn this command")
            .stdout
            .expect("Could not capture standard output.");
        let reader = BufReader::new(stdout);
        reader.lines()
            .filter_map(|line| line.ok())
            .for_each(|line| println!("{}", line));
    }
}