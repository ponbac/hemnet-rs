use anyhow::Result;

mod client;
mod listing;
mod models;
mod storage;

use client::HemnetClient;
use models::SaleCard;

const LOCATIONS: &[&str] = &[
    "474882", "474876", "944607", "474880", "947428", "474881", "474879",
];

#[tokio::main]
async fn main() -> Result<()> {
    let client = HemnetClient::new()?;
    let location_ids: Vec<String> = LOCATIONS.iter().map(|&s| s.to_string()).collect();

    // Get sold listings
    let sold_listings: Vec<SaleCard> =
        listing::fetch_all_listings(&client, 1, None, &location_ids).await?;
    println!("Found total {} sold listings", sold_listings.len());

    // Convert to CSV rows and save
    let csv_rows: Vec<_> = sold_listings
        .iter()
        .filter_map(|listing| match listing.to_csv_row() {
            Ok(row) => Some(row),
            Err(e) => {
                println!(
                    "Error converting listing to CSV row: {}, listing: {:?}",
                    e, listing
                );
                None
            }
        })
        .collect();

    storage::save_listings_to_csv(&csv_rows, "sold")?;

    Ok(())
}
