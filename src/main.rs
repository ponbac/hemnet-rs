use anyhow::Result;

mod client;
mod listing;
mod models;
mod storage;

use client::HemnetClient;
use models::SaleCard;

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
    let client = HemnetClient::new()?;

    let mut csv_rows = Vec::new();
    for (id, name) in LOCATIONS {
        let listings: Vec<SaleCard> =
            listing::fetch_all_listings(&client, 1, Some(1), &[id]).await?;
        println!("Found {} listings for {}", listings.len(), name);

        csv_rows.extend(
            listings
                .into_iter()
                .filter_map(|l| match l.to_csv_row(Some(name)) {
                    Ok(row) => Some(row),
                    Err(e) => {
                        println!(
                            "Error converting listing to CSV row: {}, listing: {:?}",
                            e, l
                        );
                        None
                    }
                }),
        );
    }

    storage::save_listings_to_csv(&csv_rows, "sold")?;

    Ok(())
}
