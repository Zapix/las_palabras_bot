use anyhow::Result;
use clap::Parser;

use las_palabras_bot::application::get_connection_pool;
use las_palabras_bot::configuration::Settings;
use las_palabras_bot::domain::verbs::repository::{VerbsDb, VerbsRepository};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_name = "PAGE", default_value_t = 0)]
    pub page: u64,

    #[arg(long, value_name = "PER_PAGE", default_value_t = 20)]
    pub per_page: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Fetching verbs...");
    let args = Args::parse();

    let setting = Settings::load().expect("Failed to load settings");
    let db_pool = get_connection_pool(&setting.database);
    let verbs_db = VerbsDb::new(&db_pool);

    println!(
        "Fetching page {} with {} verbs per page",
        args.page, args.per_page
    );

    let verbs = verbs_db.list_verbs(args.page, args.per_page).await?;
    for verb in verbs {
        println!("{:?}", verb);
    }

    let total_count = verbs_db.count_words().await?;
    let total_pages = (total_count as f64 / args.per_page as f64).ceil() as u64;

    println!("Total verbs: {}, Total pages: {}", total_count, total_pages);

    Ok(())
}
