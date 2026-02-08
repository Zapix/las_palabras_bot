use anyhow::Result;

use las_palabras_bot::application::get_connection_pool;
use las_palabras_bot::configuration::Settings;
use las_palabras_bot::domain::vocabulary::repository::{VocabularyDb, VocabularyTrait};

#[tokio::main]
async fn main() -> Result<()> {
    println!("Fetching 4 random words...");

    let settings = Settings::load().expect("Failed to load settings");
    let db_pool = get_connection_pool(&settings.database);
    let vocabulary_db = VocabularyDb::new(&db_pool);

    let words = vocabulary_db.list_random_words(4).await?;
    for word in words {
        println!("{:?}", word);
    }

    Ok(())
}
