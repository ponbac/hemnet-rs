use anyhow::Result;
use rusqlite::{params, Connection};

use crate::models::CsvRow;

pub fn init_db() -> Result<Connection> {
    let data_dir = std::path::Path::new("data");
    std::fs::create_dir_all(data_dir)?;
    let db_path = data_dir.join("listings.db");

    let conn = Connection::open(db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS listings (
            id TEXT PRIMARY KEY,
            listing_id TEXT NOT NULL,
            street_address TEXT NOT NULL,
            sold_at TEXT NOT NULL,
            sold_at_label TEXT NOT NULL,
            asking_price INTEGER,
            final_price INTEGER,
            living_area REAL,
            location TEXT,
            location_description TEXT NOT NULL,
            fee INTEGER,
            square_meter_price INTEGER,
            rooms TEXT NOT NULL,
            price_change REAL,
            broker_agency_name TEXT NOT NULL,
            broker_name TEXT,
            labels TEXT NOT NULL,
            url TEXT NOT NULL
        )",
        [],
    )?;

    Ok(conn)
}

pub fn save_listings_to_db(conn: &Connection, listings: &[CsvRow]) -> Result<()> {
    if listings.is_empty() {
        println!("No listings to save to database");
        return Ok(());
    }

    let tx = conn.transaction()?;

    for listing in listings {
        tx.execute(
            "INSERT OR REPLACE INTO listings (
                id, listing_id, street_address, sold_at, sold_at_label,
                asking_price, final_price, living_area, location,
                location_description, fee, square_meter_price, rooms,
                price_change, broker_agency_name, broker_name, labels, url
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                listing.id,
                listing.listing_id,
                listing.street_address,
                listing.sold_at,
                listing.sold_at_label,
                listing.asking_price,
                listing.final_price,
                listing.living_area,
                listing.location,
                listing.location_description,
                listing.fee,
                listing.square_meter_price,
                listing.rooms,
                listing.price_change,
                listing.broker_agency_name,
                listing.broker_name,
                listing.labels,
                listing.url,
            ],
        )?;
    }

    tx.commit()?;
    println!("Saved {} listings to database", listings.len());

    Ok(())
}