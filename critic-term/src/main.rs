use std::path::PathBuf;

use clap::{Parser, Subcommand};

use critic::prelude::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
struct AddCommand {
    /// Primary Category
    #[arg(index = 1)]
    category_db: PathBuf,
    /// Name of the entry
    #[arg(short, long)]
    name: String,

    /// List of sub-categories
    #[arg(short, long, value_parser, num_args = 0.., value_delimiter=',')]
    sub_categories: Vec<String>,

    /// Debug settings
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

#[derive(Parser, Debug)]
struct RateCommand {
    /// Primary Category to rate
    #[arg(short, long)]
    category: PathBuf,

    /// Debug settings
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

#[derive(Subcommand, Debug)]
enum Commands {
    // Adds an entry to category
    Add(AddCommand),
    Rate(RateCommand),
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add(AddCommand {
            category_db,
            name,
            sub_categories,
            debug,
        }) => {
            println!("{:?}", category_db);
            println!("{}", name);
            for i in sub_categories.iter().filter(|x| !x.is_empty()) {
                println!("Subcategory: {}", i);
            }
            match debug {
                0 => println!("Debug mode is off"),
                1 => println!("Debug mode is kind of on"),
                2 => println!("Debug mode is on"),
                _ => println!("Don't be crazy"),
            }

            let item = CategoryItem {
                name: name.to_string(),
                sub_categories: sub_categories.to_vec(),
            };

            let mut conn =
                Connection::open_category(category_db).expect("Expected Category DB to be opened");

            let _ = conn.save(&item).expect("Expected item to be saved");
        }
        Commands::Rate(RateCommand {
            category: _,
            debug: _,
        }) => {}
    }
}
