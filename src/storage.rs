use anyhow::Result;
use chrono::Local;
use std::{fs, io::Write, path::Path};

use crate::db;

pub fn save_listings<T>(listings: &[T], filename_prefix: &str) -> Result<()>
where
    T: serde::Serialize,
{
    // Initialize SQLite database
    let conn = db::init_db()?;
    if let Ok(csv_rows) = serde_json::to_value(listings) {
        if let Ok(csv_rows) = serde_json::from_value(csv_rows) {
            db::save_listings_to_db(&conn, &csv_rows)?;
        }
    }

    // Save to CSV as well
    save_listings_to_csv(listings, filename_prefix)?;
    Ok(())
}

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

    let mut file = fs::File::create(&filename)?;
    file.write_all(&[0xEF, 0xBB, 0xBF])?; // BOM for Excel compatibility

    let mut wtr = csv::WriterBuilder::new()
        .has_headers(true)
        .delimiter(b';')
        .from_writer(file);

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
