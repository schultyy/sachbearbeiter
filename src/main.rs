use jq_lang::to_ast;
use clap::{AppSettings, Clap};
use color_eyre::eyre::Result;
use color_eyre::eyre::eyre;
use std::io::{self, BufRead};

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

fn main() -> Result<()> {
    color_eyre::install()?;
    let opts: Opts = Opts::parse();

    match to_ast(&opts.filter) {
        Ok(ast) => {
            let input_str = read_input_stream();
            println!("{}", input_str);
            Ok(())
        },
        Err(err) => {
            // Err((&err).eyre_kind().new(err))
            Err(eyre!(err.to_string()))
        }
    }
}
