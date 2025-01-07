use std::sync::Arc;

use anyhow::Result;
use futures::future::join_all;

mod client;
mod db;
mod listing;
mod models;
mod storage;

use client::HemnetClient;
use models::{CsvRow, SaleCard};

const LOCATIONS: &[(&str, &str)] = &[
    ("474879", "Västermalm-Norrmalm"),
    ("474881", "Stenstaden"),
    ("947428", "Haga"),
    ("474880", "Södermalm-Östermalm"),
    ("944607", "Granlo"),
    ("474876", "Sidsjö-Sallyhill"),
    ("474882", "Skönsmon"),
];

#[tokio::main]
async fn main() -> Result<()> {
    let client = Arc::new(HemnetClient::new()?);

    // Create futures for all locations
    let futures = LOCATIONS.iter().map(|(id, name)| {
        let cloned_client = client.clone();
        tokio::spawn(async move {
            println!("Fetching sold listings for {}", name);
            let listings: Vec<SaleCard> =
                listing::fetch_all_listings(&cloned_client, name, 1, None, &[id], true).await?;
            println!("Found {} listings for {}", listings.len(), name);

            Ok::<_, anyhow::Error>(listings_to_csv_rows(listings, name))
        })
    });

    // Wait for all futures to complete
    let csv_rows = join_all(futures)
        .await
        .into_iter()
        .filter_map(|join_result| match join_result {
            Ok(location_result) => match location_result {
                Ok(rows) => Some(rows),
                Err(e) => {
                    eprintln!("Error fetching location: {}", e);
                    None
                }
            },
            Err(e) => {
                eprintln!("Task failed: {}", e);
                None
            }
        })
        .flatten()
        .collect::<Vec<_>>();

    storage::save_listings(&csv_rows, "sold")?;

    Ok(())
}

fn listings_to_csv_rows(listings: Vec<SaleCard>, location: &str) -> Vec<CsvRow> {
    listings
        .into_iter()
        .filter_map(|l: SaleCard| match l.to_csv_row(Some(location)) {
            Ok(row) => Some(row),
            Err(e) => {
                println!(
                    "Error converting listing to CSV row: {}, listing: {:?}",
                    e, l
                );
                None
            }
        })
        .collect()
}
