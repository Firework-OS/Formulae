/*
 * Copyright (c) VisualDevelopment 2021-2022.
 * This project is licensed by the Creative Commons Attribution-NoCommercial-NoDerivatives licence.
 */

#![deny(warnings, unused_extern_crates, clippy::cargo, rust_2021_compatibility)]

use std::io::{BufWriter, Write};

use clap::{AppSettings, Parser, Subcommand};
use hashbrown::HashMap;

#[derive(Parser)]
#[clap(name = "ftool")]
#[clap(about = "Formulae binary configuration format manipulation example CLI", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(setting(AppSettings::ArgRequiredElseHelp))]
    New {
        #[clap(required = true, parse(from_os_str))]
        filename: std::path::PathBuf,
    },
    Read {
        #[clap(required = true, parse(from_os_str))]
        filename: std::path::PathBuf,
    },
}

fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::New { filename } => {
            let mut data = formulae::Root::default();
            data.insert("Cool".to_string(), formulae::Node::Bool(true));
            data.insert("somenumber".to_string(), formulae::Node::Int64(0xABCDEF));
            data.insert(
                "A string".to_string(),
                formulae::Node::String("hello world".to_string()),
            );
            data.insert(
                "array".to_string(),
                formulae::Node::Array(vec![
                    formulae::Node::String("hello world".to_string()),
                    formulae::Node::Int64(0xABCDEF),
                ]),
            );
            let mut map = HashMap::new();
            map.insert("macos".to_string(), formulae::Node::Bool(true));
            map.insert("me".to_string(), formulae::Node::Bool(true));
            map.insert("microsoft".to_string(), formulae::Node::Bool(false));
            data.insert(
                "cool_things_def".to_string(),
                formulae::Node::Dictionary(map),
            );
            BufWriter::new(std::fs::File::create(filename).unwrap())
                .write(&data.into_bytes())
                .unwrap();
        }
        Commands::Read { filename } => {
            let contents = std::fs::read(filename).unwrap();
            println!("{:#X?}", formulae::Root::parse(&contents));
        }
    }
}
