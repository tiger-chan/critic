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

fn save_match(
    conn: &mut Connection,
    contest: &critic::dto::Contest,
    score: f32,
) -> Result<(), critic::DbError> {
    let result = critic::dto::MatchResult {
        score,
        a: contest.a.id,
        b: contest.b.id,
        criterion: contest.category.id,
        elo_change: critic::elo::calc_change(contest.a.elo, contest.b.elo, score),
    };
    conn.save(&result).map(|_| ())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
            let mut conn =
                Connection::open_category(category_db).expect("Expected Category DB to be opened");

            let mut show_help = false;
            let mut contest = conn.next_contest()?;
            'contest: loop {
                use io::Write;
                let mut stdout = io::stdout();

                stdout.execute(terminal::Clear(terminal::ClearType::All))?;
                stdout.execute(cursor::MoveTo(0, 0))?;

                println!(r#"Criterion: "{}""#, contest.category.name);
                println!("1. {}", contest.a.name);
                println!("2. {}", contest.b.name);

                if show_help {
                    println!("----------------------");
                    println!("e. Equal");
                    println!("s. Skip for now");
                    println!("q. Quit");
                    println!("?. Display this help");
                    show_help = false;
                }

                print!("Which is better? (1/2/e/s/q/?): ");

                stdout.flush()?;
                'evt: loop {
                    if poll(Duration::from_millis(100))? {
                        match event::read() {
                            Ok(event::Event::Key(evt)) => match evt.code {
                                event::KeyCode::Char('1') => {
                                    save_match(&mut conn, &contest, 1.0)?;
                                    contest = conn.next_contest()?;
                                    break 'evt;
                                }
                                event::KeyCode::Char('2') => {
                                    save_match(&mut conn, &contest, 0.0)?;
                                    contest = conn.next_contest()?;
                                    break 'evt;
                                }
                                event::KeyCode::Char('e') => {
                                    save_match(&mut conn, &contest, 0.5)?;
                                    contest = conn.next_contest()?;
                                    break 'evt;
                                }
                                event::KeyCode::Char('s') => {
                                    contest = conn.next_contest()?;
                                    break 'evt;
                                }
                                event::KeyCode::Char('q') => {
                                    break 'contest;
                                }
                                event::KeyCode::Char('?') => {
                                    show_help = true;
                                    break 'evt;
                                }
                                _ => {}
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

    Ok(())
}
