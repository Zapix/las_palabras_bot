use clap::Parser;
use tokio::fs::{File, try_exists};
use tokio::io::AsyncReadExt;

use las_palabras_bot::application::get_connection_pool;
use las_palabras_bot::configuration::Settings;
use las_palabras_bot::domain::verbs::raw_verb::RawVerb;
use las_palabras_bot::domain::verbs::repository::{VerbsDb, VerbsRepository};

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
    match try_exists(&args.file).await {
        Ok(true) => {
            println!("File exists: {}", args.file);
        }
        Ok(false) => {
            return Err(anyhow::anyhow!("File does not exist: {}", args.file));
        }
        Err(e) => {
            return Err(anyhow::anyhow!("Failed to check file existence: {}", e));
        }
    }
    let mut file = File::open(args.file).await?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).await?;

    let content = serde_json::from_str::<Vec<RawVerb>>(&buffer)
        .map_err(|e| anyhow::anyhow!("Failed to parse JSON: {}", e))?;

    println!("Parsed {} verbs from the verbs file.", content.len());

    let settings = Settings::load().expect("Failed to load settings");
    let connection = get_connection_pool(&settings.database);
    let verbs_db = VerbsDb::new(&connection);
    let new_verbs = verbs_db.add_batch_verbs(content).await?;

    println!("Saved {} verbs into db", new_verbs.len());
    Ok(())
}
