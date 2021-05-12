use regex::Regex;
use std::path::PathBuf;
use std::process::Command;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opts {
    fname: PathBuf,
}

fn main() {
    let opt = Opts::from_args();
    let output = Command::new("satysfi")
        .args(&[
            opt.fname.to_string_lossy().into_owned(),
            "--type-check-only".to_owned(),
        ])
        .output()
        .expect("failed to execute satysfi");
    let output = std::str::from_utf8(&output.stdout).unwrap();
    let lines: Vec<_> = output.split('\n').collect();

    if let Some(TypeErrorMessage {
        file,
        line,
        characters,
        text,
    }) = extract_type_error(&lines)
    {
        println!("{}:{}:{}: {}", file, line, characters, text);
        std::process::exit(-1);
    }
}

struct TypeErrorMessage {
    file: String,
    line: usize,
    characters: usize,
    text: String,
}

fn extract_type_error(block: &[&str]) -> Option<TypeErrorMessage> {
    let regex_type_error = Regex::new(r"^! \[Type Error\]").unwrap();
    let regex_file = Regex::new(r#""(.*)""#).unwrap();
    // TODO: capture range
    let regex_line = Regex::new(r"line (\d+)").unwrap();
    let regex_chars = Regex::new(r"characters (\d+)").unwrap();
    block
        .iter()
        .enumerate()
        .find(|(_, &s)| regex_type_error.find(s).is_some())
        .and_then(|(idx, &s)| {
            let file = regex_file.captures(s);
            if file.is_none() {
                eprintln!("filename pattern not found.");
                return None;
            }
            let file = file.unwrap().get(1).unwrap().as_str().to_owned();

            let line = regex_line.captures(s);
            if line.is_none() {
                eprintln!("linename pattern not found.");
                return None;
            }
            let line: usize = line.unwrap().get(1).unwrap().as_str().parse().unwrap();

            let characters = regex_chars.captures(s);
            if characters.is_none() {
                eprintln!("characters pattern not found.");
                return None;
            }
            let characters: usize = characters
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse()
                .unwrap();
            let text = block[idx + 1..]
                .iter()
                .map(|&s| s.trim_start())
                .collect::<Vec<_>>()
                // .join("\n");
                .join(" ");
            Some(TypeErrorMessage {
                file,
                line,
                characters,
                text,
            })
        })
}
