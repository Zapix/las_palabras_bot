use anyhow::Result;
use clap::Parser;

use las_palabras_bot::application::get_connection_pool;
use las_palabras_bot::configuration::Settings;
use las_palabras_bot::domain::vocabulary::repository::{VocabularyDb, VocabularyTrait};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, value_name = "UUID", required = true, value_delimiter = ',')]
    ids: Vec<uuid::Uuid>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    println!("Fetching words for {} ids...", args.ids.len());

    let settings = Settings::load().expect("Failed to load settings");
    let db_pool = get_connection_pool(&settings.database);
    let vocabulary_db = VocabularyDb::new(&db_pool);

    let words = vocabulary_db.list_word_by_ids(&args.ids).await?;
    for word in words {
        println!("{:?}", word);
    }

    Ok(())
}
