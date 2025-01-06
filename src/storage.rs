use anyhow::Result;
use chrono::Local;
use std::{fs, io::Write, path::Path};

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
