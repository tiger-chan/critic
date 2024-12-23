mod app;

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Category Database
    #[arg(index = 1)]
    category_db: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut terminal = ratatui::init();
    let result = app::App::new(args.category_db).run(&mut terminal);
    ratatui::restore();
    result
}
