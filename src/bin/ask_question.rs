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
    id: uuid::Uuid,
    #[arg(short, long, default_value_t = false)]
    debug: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let settings = Settings::load().expect("Failed to load settings");
    let db_pool = get_connection_pool(&settings.database);
    let mut game_db = GameDb::new(&db_pool);

    let game = game_db.ask_question(args.id).await?;
    if args.debug {
        println!("{:?}", game);
    } else {
        println!("Question asked for game {}", args.id);
        let vocabulary_db = VocabularyDb::new(&db_pool);
        let converter = Converter::new(&vocabulary_db);
        println!("{:?}", converter.convert_game_to_output(&game).await?);
    }

    Ok(())
}
