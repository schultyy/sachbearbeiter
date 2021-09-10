use jq_lang::to_ast;
use clap::{AppSettings, Clap};
use color_eyre::eyre::Result;
use color_eyre::eyre::eyre;
use std::io::{self, BufRead};
use serde::Deserialize;
use serde_json::Value;

#[derive(Clap)]
#[clap(version = "1.0", author = "Jan Schulte")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    filter: String
}

fn read_input_stream() -> String {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().collect::<Vec<_>>();

    let mut results = vec!();

    for line in lines {
        let line = line.expect("Could not read line from standard in");
        results.push(line);
    }

    results.join("\n")
}

fn parse_json(data: &str) -> serde_json::Result<Value> {
    let value = serde_json::from_str(data)?;
    Ok(value)
}

fn traverse_ast() -> serde_json::Result<String> {
    let input_str = parse_json(&read_input_stream())?;
    let json_str = serde_json::to_string(&input_str)?;
    Ok(json_str)
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let opts: Opts = Opts::parse();

    match to_ast(&opts.filter) {
        Ok(ast) => {
            match traverse_ast() {
                Ok(json) => {
                    println!("{}", json);
                    Ok(())
                },
                Err(err) => Err(eyre!(err.to_string()))
            }
        },
        Err(err) => {
            Err(eyre!(err.to_string()))
        }
    }
}
