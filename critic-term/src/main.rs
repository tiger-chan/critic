use clap::{Parser, Subcommand};
use crossterm::{
    cursor,
    event::{self, poll},
    terminal, ExecutableCommand,
};
use std::{io, path::PathBuf, time::Duration};

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
    #[arg(index = 1)]
    category_db: PathBuf,

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

fn main() -> std::io::Result<()> {
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

            let item = NewCategoryItem {
                name: name.to_string(),
                sub_categories: sub_categories.to_vec(),
            };

            let mut conn =
                Connection::open_category(category_db).expect("Expected Category DB to be opened");

            let _ = conn.save(&item).expect("Expected item to be saved");
        }
        Commands::Rate(RateCommand {
            category_db,
            debug: _,
        }) => {
            let conn =
                Connection::open_category(category_db).expect("Expected Category DB to be opened");

            'contest: loop {
                if let Ok(contest) = conn.next_contest() {
                    use io::Write;
                    let mut stdout = io::stdout();

                    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
                    stdout.execute(cursor::MoveTo(0, 0))?;

                    println!(r#"Criterion: "{}""#, contest.category.name);
                    println!("1. {}", contest.a.name);
                    println!("2. {}", contest.b.name);

                    print!("Which is better? (1/2/(s)kip/(q)quit): ");

                    stdout.flush()?;
                    'evt: loop {
                        if poll(Duration::from_millis(100))? {
                            match event::read() {
                                Ok(event::Event::Key(evt)) => match evt.code {
                                    event::KeyCode::Char('1') => {
                                        println!("Is 1 really better than 2?");
                                        break 'contest;
                                    }
                                    event::KeyCode::Char('2') => {
                                        println!("Is 2 really better than 1?");
                                        break 'contest;
                                    }
                                    event::KeyCode::Char('s') => {
                                        println!("Moving On...");
                                        break 'evt;
                                    }
                                    event::KeyCode::Char('q') => {
                                        break 'contest;
                                    }
                                    _ => {
                                        stdout
                                            .execute(terminal::Clear(terminal::ClearType::All))?;
                                        stdout.execute(cursor::MoveTo(0, 0))?;

                                        println!(r#"Criterion: "{}""#, contest.category.name);
                                        println!("1. {}", contest.a.name);
                                        println!("2. {}", contest.b.name);

                                        print!("Which is better? (1/2/(s)kip/(q)quit): ");

                                        stdout.flush()?;
                                    }
                                },
                                _ => {
                                    println!("Unexpected event");
                                    break 'contest;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
