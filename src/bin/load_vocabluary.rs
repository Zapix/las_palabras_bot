use clap::Parser;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use las_palabras_bot::domain::vocabluary::raw_word::RawWord;

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
    Ok(())
}
