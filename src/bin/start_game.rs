use clap::Parser;

use las_palabras_bot::application::get_connection_pool;
use las_palabras_bot::configuration::Settings;
use las_palabras_bot::domain::word_game::GameType;
use las_palabras_bot::domain::word_game::repository::{GameDb, GameTrait};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_name = "GAME_TYPE")]
    game_type: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Args = Args::parse();
    let game_type: GameType = args.game_type.into();
    println!("Game type: {}", game_type);

    let settings = Settings::load().expect("Failed to load settings");
    let connection = get_connection_pool(&settings.database);
    let mut game_db = GameDb::new(&connection);
    let game = game_db.new_game(game_type).await?;

    println!("New game created: {:?}", game);

    Ok(())
}
