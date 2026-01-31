use anyhow::Result;
use clap::Parser;

use las_palabras_bot::application::get_connection_pool;
use las_palabras_bot::configuration::Settings;
use las_palabras_bot::domain::word_game::repository::{GameDb, GameTrait};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_name = "UUID")]
    id: uuid::Uuid,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let settings = Settings::load().expect("Failed to load settings");
    let db_pool = get_connection_pool(&settings.database);
    let mut game_db = GameDb::new(&db_pool);

    match game_db.load_game_by_id(args.id).await? {
        Some(game) => println!("{:?}", game),
        None => println!("Game not found: {}", args.id),
    }

    Ok(())
}
