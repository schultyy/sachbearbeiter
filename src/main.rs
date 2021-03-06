use crate::traverse_error::TraverseError;
use jq_lang::{to_ast, node_type::NodeType};
use clap::{AppSettings, Clap};
use color_eyre::eyre::Result;
use color_eyre::eyre::eyre;
use std::io::{self, BufRead};
use serde_json::Value;

mod traverse_error;

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

fn traverse_ast(ast: jq_lang::node::Node) -> Result<String, Box<dyn std::error::Error>> {
    let child_node = &ast.children.unwrap()[0];
    let object_tree = parse_json(&read_input_stream())?;

    match child_node.node_type {
        NodeType::Identity => {
            let json_str = serde_json::to_string(&object_tree)?;
            Ok(json_str)
        },
        NodeType::ObjectIdentifierIndex => {
            let key = &child_node.value.as_ref().unwrap();
            if let Some(value) = object_tree.get(key) {
                Ok(value.to_string())
            }
            else {
                Err(Box::new(TraverseError::new(format!("Could not find key '{}' in object", key))))
            }
        },
        _ => {
            Ok(String::new())
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let opts: Opts = Opts::parse();

    match to_ast(&opts.filter) {
        Ok(ast) => {
            match traverse_ast(ast) {
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
