use anyhow::Result;
use clap::Parser;

use las_palabras_bot::application::get_connection_pool;
use las_palabras_bot::configuration::Settings;
use las_palabras_bot::domain::verbs::repository::{VerbsDb, VerbsRepository};
use las_palabras_bot::domain::verbs::verb::Verb;

fn print_verb(verb: &Verb) {
    println!("ID: {}", verb.id);
    println!("Verb: {}", verb.verb);
    println!("Created At: {}", verb.created_at);
    println!("Updated At: {}", verb.updated_at);
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, value_name = "ID")]
    pub id: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let setting = Settings::load().expect("Failed to load settings");
    let db_pool = get_connection_pool(&setting.database);
    let verbs_db = VerbsDb::new(&db_pool);

    let verb_id = uuid::Uuid::parse_str(&args.id)?;

    let verb = verbs_db.get_verb_by_id(verb_id).await?;

    match verb {
        Some(v) => print_verb(&v),
        None => println!("No verb found with ID: {}", args.id),
    }

    Ok(())
}
