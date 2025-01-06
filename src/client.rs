use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue};
use std::collections::HashMap;

use crate::models::{HemnetListingsResponse, ListingCard, SaleCard};

const BASE_URL: &str = "https://www.hemnet.se/_next/data/ZbTIGtigbip8_BxHWbd_z";

pub struct HemnetClient {
    client: reqwest::Client,
}

impl HemnetClient {
    pub fn new() -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert("accept", HeaderValue::from_static("*/*"));
        headers.insert(
            "accept-language",
            HeaderValue::from_static("sv-SE,sv;q=0.5"),
        );
        headers.insert("cookie", HeaderValue::from_static("force-light-mode=true; hn_uc_consent={}; hn_exp_kpis=366; hn_exp_noi=655; hn_exp_bau=698; hn_exp_copc=667; hn_exp_prd=640; hn_exp_nhc=798; __cfruid=cfc84fa0bbd11dc60cb72bb426ffc133c9909235-1735994010; CF_AppSession=n95f4e1a3f7fd2f56"));
        headers.insert("priority", HeaderValue::from_static("u=1, i"));
        headers.insert(
            "referer",
            HeaderValue::from_static("https://www.hemnet.se/bostader"),
        );
        headers.insert(
            "sec-ch-ua",
            HeaderValue::from_static(
                "\"Brave\";v=\"131\", \"Chromium\";v=\"131\", \"Not_A Brand\";v=\"24\"",
            ),
        );
        headers.insert("sec-ch-ua-mobile", HeaderValue::from_static("?0"));
        headers.insert(
            "sec-ch-ua-platform",
            HeaderValue::from_static("\"Windows\""),
        );
        headers.insert("sec-fetch-dest", HeaderValue::from_static("empty"));
        headers.insert("sec-fetch-mode", HeaderValue::from_static("cors"));
        headers.insert("sec-fetch-site", HeaderValue::from_static("same-origin"));
        headers.insert("sec-gpc", HeaderValue::from_static("1"));
        headers.insert("user-agent", HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36"));
        headers.insert("x-nextjs-data", HeaderValue::from_static("1"));

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self { client })
    }

    pub async fn get_listings(&self, location_ids: &[&str], page: u32) -> Result<Vec<ListingCard>> {
        let mut params = HashMap::new();
        params.insert("item_types[]", "bostadsratt");
        params.insert("living_area_min", "40");
        let page_str = page.to_string();
        params.insert("page", &page_str);

        for location_id in location_ids {
            params.insert("location_ids[]", location_id);
        }

        let url = format!("{}/bostader.json", BASE_URL);
        let response = self
            .client
            .get(&url)
            .query(&params)
            .send()
            .await?
            .error_for_status()?;

        let data: HemnetListingsResponse = response.json().await?;
        let apollo_state = data.page_props.apollo_state;

        let mut listings = Vec::new();
        if let Some(obj) = apollo_state.extra.as_object() {
            for (key, value) in obj {
                if key.starts_with("ListingCard:") {
                    if let Ok(listing) = serde_json::from_value(value.clone()) {
                        listings.push(listing);
                    }
                }
            }
        }

        Ok(listings)
    }

    pub async fn get_sold_listings(
        &self,
        location_ids: &[&str],
        page: u32,
    ) -> Result<Vec<SaleCard>> {
        let mut params = HashMap::new();
        params.insert("item_types[]", "bostadsratt");
        params.insert("living_area_min", "40");
        let page_str = page.to_string();
        params.insert("page", &page_str);

        for location_id in location_ids {
            params.insert("location_ids[]", location_id);
        }

        let url = format!("{}/salda/bostader.json", BASE_URL);
        let response = self
            .client
            .get(&url)
            .query(&params)
            .send()
            .await?
            .error_for_status()?;

        let raw_data = response.text().await?;
        let data: HemnetListingsResponse = serde_json::from_str(&raw_data)?;

        let mut sold_listings = Vec::new();
        if let Some(obj) = data.page_props.apollo_state.extra.as_object() {
            for (key, value) in obj {
                if key.starts_with("SaleCard:") {
                    if let Ok(sale) = serde_json::from_value(value.clone()) {
                        sold_listings.push(sale);
                    } else {
                        println!("Error converting sale to SaleCard: {:?}", value);
                    }
                }
            }
        }

        if sold_listings.is_empty() {
            println!(
                "No sold listings found, response: {}",
                serde_json::to_string_pretty(&raw_data)?
            );
        }

        Ok(sold_listings)
    }
}
