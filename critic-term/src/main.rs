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

    /// List of groupings of criteria to add.
    #[arg(short, long, value_parser, num_args = 0.., value_delimiter=',')]
    criteria_group: Vec<String>,
    // /// Debug settings
    // #[arg(short, long, action = clap::ArgAction::Count)]
    // debug: u8,
}

#[derive(Parser, Debug)]
struct TopCommand {
    /// Primary Category to rate
    #[arg(index = 1)]
    category_db: PathBuf,

    #[arg(short, long, default_value = "")]
    group: String,

    /// Number of entries to display
    #[arg(short, long, default_value = "30")]
    count: u8,

    /// Number of entries to display
    #[arg(short, long, default_value = "0")]
    page: u8,
}

#[derive(Parser, Debug)]
struct RateCommand {
    /// Primary Category to rate
    #[arg(index = 1)]
    category_db: PathBuf,
    // /// Debug settings
    // #[arg(short, long, action = clap::ArgAction::Count)]
    // debug: u8,
}

#[derive(Subcommand, Debug)]
enum Commands {
    // Adds an entry to category
    Add(AddCommand),
    Top(TopCommand),
    Rate(RateCommand),
}

fn save_match(
    conn: &mut Connection,
    contest: &critic::dto::Contest,
    score: f32,
) -> Result<(), critic::DbError> {
    let result = critic::dto::MatchResult {
        score,
        criteria_group: contest.criterion.group,
        a: contest.a.id,
        b: contest.b.id,
        criterion: contest.criterion.id,
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
            criteria_group,
        }) => {
            println!("{:?}", category_db);
            println!("{}", name);
            for i in criteria_group.iter().filter(|x| !x.is_empty()) {
                println!("Subcategory: {}", i);
            }

            let item = NewCategoryItem {
                name: name.to_string(),
                sub_categories: criteria_group.to_vec(),
            };

            let mut conn =
                Connection::open_category(category_db).expect("Expected Category DB to be opened");

            let _ = conn.save(&item).expect("Expected item to be saved");
        }
        Commands::Top(TopCommand {
            category_db,
            group,
            count,
            page,
        }) => {
            let conn =
                Connection::open_category(category_db).expect("Expected Category DB to be opened");

            let rows = conn.top(group.as_str(), *count as usize, *page as usize)?;

            println!("'{}', c: {}, p:{} Rows found: {}", group, count, page, rows.len());
            for row in rows {
                println!(
                    "Group: {}, Entry: {}, ELO: {}",
                    row.group, row.entry, row.elo
                );
            }
        }
        Commands::Rate(RateCommand { category_db }) => {
            let mut conn =
                Connection::open_category(category_db).expect("Expected Category DB to be opened");

            let mut show_help = false;
            let mut contest = conn.next_contest()?;
            'contest: loop {
                use io::Write;
                let mut stdout = io::stdout();

                stdout.execute(terminal::Clear(terminal::ClearType::All))?;
                stdout.execute(cursor::MoveTo(0, 0))?;

                println!(
                    r#"Criterion: {}: "{}""#,
                    contest.criterion.group_name, contest.criterion.name
                );
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
