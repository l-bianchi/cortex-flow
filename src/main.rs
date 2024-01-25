use clap::{Parser, Subcommand};
use dotenv::dotenv;
use meilisearch_sdk::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Search { prompt: String },
    Index { title: String },
    Clean { index: String },
}

#[derive(Serialize, Deserialize, Debug)]
struct Movie {
    id: String,
    title: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cli = Cli::parse();

    let meilisearch_url = std::env::var("MEILISEARCH_URL").expect("MEILISEARCH_URL must be set.");
    let meilisearch_api_key =
        std::env::var("MEILISEARCH_API_KEY").expect("MEILISEARCH_API_KEY must be set.");

    let client = Client::new(meilisearch_url, Some(meilisearch_api_key));

    match &cli.command {
        Some(Commands::Search { prompt }) => {
            let results: SearchResults<Movie> = client
                .index("movies")
                .search()
                .with_query(prompt)
                .execute()
                .await
                .unwrap();
            println!("{:?}", results.hits);
        }
        Some(Commands::Index { title }) => {
            client
                .index("movies")
                .add_documents(
                    &[Movie {
                        id: Uuid::new_v4().to_string(),
                        title: String::from(title),
                    }],
                    Some("id"),
                )
                .await
                .unwrap();
        }
        Some(Commands::Clean { index }) => {
            client.index(index).delete().await.unwrap();
        }
        None => {}
    }
}
