use anyhow::Result;
use rand::Rng;
use std::time::Duration;

use crate::{
    client::HemnetClient,
    models::{ListingCard, SaleCard},
};

pub trait Listing: serde::Serialize {
    fn id(&self) -> &str;

    async fn fetch_page(
        client: &HemnetClient,
        location_ids: &[&str],
        page: u32,
    ) -> Result<Vec<Self>>
    where
        Self: Sized;
}

impl Listing for ListingCard {
    fn id(&self) -> &str {
        &self.id
    }

    async fn fetch_page(
        client: &HemnetClient,
        location_ids: &[&str],
        page: u32,
    ) -> Result<Vec<Self>> {
        client.get_listings(location_ids, page).await
    }
}

impl Listing for SaleCard {
    fn id(&self) -> &str {
        &self.listing_id
    }

    async fn fetch_page(
        client: &HemnetClient,
        location_ids: &[&str],
        page: u32,
    ) -> Result<Vec<Self>> {
        client.get_sold_listings(location_ids, page).await
    }
}

pub async fn fetch_all_listings<T: Listing>(
    client: &HemnetClient,
    location_name: &str,
    start_page: u32,
    max_page: Option<u32>,
    location_ids: &[&str],
    random_sleep: bool,
) -> Result<Vec<T>> {
    let mut listings = Vec::new();
    let mut page = start_page;

    while max_page.map_or(true, |max| page <= max) {
        let page_listings = T::fetch_page(client, location_ids, page).await?;
        println!(
            "{}: Found {} listings on page {}, total listings: {}",
            location_name,
            page_listings.len(),
            page,
            listings.len() + page_listings.len()
        );

        let unique_listings: Vec<_> = page_listings
            .into_iter()
            .filter(|listing| {
                let is_duplicate = listings.iter().any(|l: &T| l.id() == listing.id());
                if is_duplicate {
                    println!("Duplicate listing found: {}", listing.id());
                }
                !is_duplicate
            })
            .collect();

        if unique_listings.is_empty() {
            break;
        }

        listings.extend(unique_listings);
        page += 1;

        if random_sleep {
            let wait_time = rand::thread_rng().gen_range(1000..5000);
            tokio::time::sleep(Duration::from_millis(wait_time)).await;
        }
    }

    Ok(listings)
}
