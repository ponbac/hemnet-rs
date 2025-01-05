use anyhow::Result;
use chrono::Local;
use std::{fs, path::Path};

pub fn save_listings_to_csv<T>(listings: &[T], filename_prefix: &str) -> Result<()>
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