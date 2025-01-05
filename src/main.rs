use anyhow::Result;
use chrono::Local;
use rand::Rng;
use std::{fs, path::Path, time::Duration};
use tokio::time::sleep;

mod client;
mod models;

use client::HemnetClient;
use models::{ListingCard, SaleCard};

const LOCATIONS: &[&str] = &[
    "474882", "474876", "944607", "474880", "947428", "474881", "474879",
];

#[async_trait::async_trait]
trait Listing: serde::Serialize {
    async fn fetch_page(
        client: &HemnetClient,
        location_ids: &[String],
        page: u32,
    ) -> Result<Vec<Self>>
    where
        Self: Sized;
}

#[async_trait::async_trait]
impl Listing for ListingCard {
    async fn fetch_page(
        client: &HemnetClient,
        location_ids: &[String],
        page: u32,
    ) -> Result<Vec<Self>> {
        client.get_listings(location_ids, page).await
    }
}

#[async_trait::async_trait]
impl Listing for SaleCard {
    async fn fetch_page(
        client: &HemnetClient,
        location_ids: &[String],
        page: u32,
    ) -> Result<Vec<Self>> {
        client.get_sold_listings(location_ids, page).await
    }
}

async fn fetch_all_listings<T: Listing>(
    client: &HemnetClient,
    start_page: u32,
    max_pages: Option<u32>,
) -> Result<Vec<T>> {
    let mut listings = Vec::new();
    let mut page = start_page;
    let location_ids: Vec<String> = LOCATIONS.iter().map(|&s| s.to_string()).collect();

    loop {
        if let Some(max) = max_pages {
            if page > max {
                break;
            }
        }

        let page_listings = T::fetch_page(client, &location_ids, page).await?;
        println!(
            "Found {} listings on page {}, total listings: {}",
            page_listings.len(),
            page,
            listings.len()
        );

        if page_listings.is_empty() {
            break;
        }

        listings.extend(page_listings);
        page += 1;

        // Sleep between 1-5 seconds before next request
        let sleep_duration = rand::thread_rng().gen_range(1..=5);
        sleep(Duration::from_secs(sleep_duration)).await;
    }

    Ok(listings)
}

fn save_listings_to_csv<T>(listings: &[T], filename_prefix: &str) -> Result<()>
where
    T: serde::Serialize,
{
    if listings.is_empty() {
        println!("No {} listings to save", filename_prefix);
        return Ok(());
    }

    let data_dir = Path::new("data");
    fs::create_dir_all(data_dir)?;

    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let filename = data_dir.join(format!("{}_{}.csv", filename_prefix, timestamp));

    let mut wtr = csv::Writer::from_path(&filename)?;
    for listing in listings {
        wtr.serialize(listing)?;
    }
    wtr.flush()?;

    println!(
        "Saved {} {} listings to {}",
        listings.len(),
        filename_prefix,
        filename.display()
    );

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = HemnetClient::new()?;

    // Get sold listings
    let sold_listings: Vec<SaleCard> = fetch_all_listings(&client, 26, None).await?;
    println!("Found total {} sold listings", sold_listings.len());

    // Convert to CSV rows and save
    let csv_rows: Vec<_> = sold_listings
        .iter()
        .filter_map(|listing| listing.to_csv_row().ok())
        .collect();
    
    save_listings_to_csv(&csv_rows, "sold")?;

    Ok(())
}
