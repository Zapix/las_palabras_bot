use clap::Parser;

use las_palabras_bot::application::get_connection_pool;
use las_palabras_bot::configuration::Settings;
use las_palabras_bot::domain::vocabluary::raw_word::RawWord;
use las_palabras_bot::domain::vocabluary::repository::VocabluaryDb;
use las_palabras_bot::domain::vocabluary::repository::VocabluaryTrait;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_name = "SPANISH")]
    spanish: String,
    #[arg(short, long, value_name = "PART_OF_SPEECH")]
    part_of_speech: String,
    #[arg(short, long, value_name = "RUSSIAN")]
    russian: String,
}

impl From<Args> for RawWord {
    fn from(args: Args) -> Self {
        RawWord {
            spanish: args.spanish,
            part_of_speech: args.part_of_speech.into(),
            russian: args.russian,
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let raw_word: RawWord = Args::parse().into();
    println!("Adding word spanish word \"{}\"", raw_word.spanish);
    let settings = Settings::load().expect("Failed to load settings");
    let connection = get_connection_pool(&settings.database);
    let vocabluary_db = VocabluaryDb::new(&connection);
    let word = vocabluary_db.create_word(raw_word).await?;
    println!("Added word: {:?}", word);
    Ok(())
}
