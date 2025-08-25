use clap::Parser;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use las_palabras_bot::application::get_connection_pool;
use las_palabras_bot::configuration::Settings;
use las_palabras_bot::domain::vocabulary::raw_word::RawWord;
use las_palabras_bot::domain::vocabulary::repository::{VocabularyDb, VocabularyTrait};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_name = "FILE")]
    file: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    println!("Received file: {}", args.file);
    let mut file = File::open(args.file).await?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).await?;

    let content = serde_json::from_str::<Vec<RawWord>>(&buffer)
        .map_err(|e| anyhow::anyhow!("Failed to parse JSON: {}", e))?;

    println!("Parsed {} words from the vocabulary file.", content.len());

    let settings = Settings::load().expect("Failed to load settings");
    let connection = get_connection_pool(&settings.database);
    let vocabluary_db = VocabularyDb::new(&connection);

    let new_words = vocabluary_db.create_batch_words(content).await?;

    for (i, word) in new_words.iter().enumerate() {
        println!("{}. {:?}", i + 1, word);
    }

    println!("Saved {} words into db", new_words.len());
    Ok(())
}
