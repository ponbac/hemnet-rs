use anyhow::Result;

use crate::{
    client::HemnetClient,
    models::{ListingCard, SaleCard},
};

pub trait Listing: serde::Serialize {
    fn id(&self) -> String;

    async fn fetch_page(
        client: &HemnetClient,
        location_ids: &[String],
        page: u32,
    ) -> Result<Vec<Self>>
    where
        Self: Sized;
}

impl Listing for ListingCard {
    fn id(&self) -> String {
        self.id.clone()
    }

    async fn fetch_page(
        client: &HemnetClient,
        location_ids: &[String],
        page: u32,
    ) -> Result<Vec<Self>> {
        client.get_listings(location_ids, page).await
    }
}

impl Listing for SaleCard {
    fn id(&self) -> String {
        self.listing_id.clone()
    }

    async fn fetch_page(
        client: &HemnetClient,
        location_ids: &[String],
        page: u32,
    ) -> Result<Vec<Self>> {
        client.get_sold_listings(location_ids, page).await
    }
}

pub async fn fetch_all_listings<T: Listing>(
    client: &HemnetClient,
    start_page: u32,
    max_page: Option<u32>,
    location_ids: &[String],
) -> Result<Vec<T>> {
    use rand::Rng;
    use std::time::Duration;
    use tokio::time::sleep;

    let mut listings = Vec::new();
    let mut page = start_page;

    while max_page.map_or(true, |max| page <= max) {
        let page_listings = T::fetch_page(client, location_ids, page).await?;
        println!(
            "Found {} listings on page {}, total listings: {}",
            page_listings.len(),
            page,
            listings.len()
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

        sleep(Duration::from_secs(rand::thread_rng().gen_range(1..=5))).await;
    }

    Ok(listings)
}