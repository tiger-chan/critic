use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
struct AddCommand {
    /// Primary Category
    #[arg(short, long)]
    category: String,
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
    /// Primary Category
    #[arg(short, long)]
    category: String,
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
            category,
            name,
            sub_categories,
            debug,
        }) => {
            println!("{}", category);
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
        }
        Commands::Rate(RateCommand { category: _ }) => {
        }
    }
}
