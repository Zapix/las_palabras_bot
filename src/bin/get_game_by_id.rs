use anyhow::Result;
use clap::Parser;

use las_palabras_bot::application::get_connection_pool;
use las_palabras_bot::configuration::Settings;
use las_palabras_bot::domain::vocabulary::repository::VocabularyDb;
use las_palabras_bot::domain::word_game::converter::Converter;
use las_palabras_bot::domain::word_game::repository::{GameDb, GameTrait};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_name = "UUID")]
    game_id: uuid::Uuid,
    #[arg(short, long, default_value_t = false)]
    debug: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let settings = Settings::load().expect("Failed to load settings");
    let db_pool = get_connection_pool(&settings.database);
    let mut game_db = GameDb::new(&db_pool);

    match game_db.load_game_by_id(args.game_id).await? {
        Some(game) if args.debug => println!("{:?}", game),
        Some(game) => {
            let vocabulary_db = VocabularyDb::new(&db_pool);
            let converter = Converter::new(&vocabulary_db);
            println!("{:?}", converter.convert_game_to_output(&game).await?);
        }
        None => println!("Game not found: {}", args.game_id),
    }

    Ok(())
}
