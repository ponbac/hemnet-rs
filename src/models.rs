use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Coordinates {
    pub lat: f64,
    pub long: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HousingForm {
    pub symbol: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Label {
    pub text: Option<String>,
    pub identifier: String,
    pub category: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListingCard {
    pub id: String,
    #[serde(rename = "activePackage")]
    pub active_package: Option<String>,
    #[serde(rename = "askingPrice")]
    pub asking_price: Option<String>,
    #[serde(rename = "brokerAgencyLogo")]
    pub broker_agency_logo: Option<String>,
    #[serde(rename = "brokerAgencyName")]
    pub broker_agency_name: Option<String>,
    pub coordinates: Coordinates,
    pub description: String,
    pub fee: Option<String>,
    pub floor: Option<String>,
    #[serde(rename = "housingForm")]
    pub housing_form: HousingForm,
    pub labels: Vec<Label>,
    #[serde(rename = "landArea")]
    pub land_area: Option<String>,
    #[serde(rename = "livingAndSupplementalAreas")]
    pub living_and_supplemental_areas: String,
    #[serde(rename = "locationDescription")]
    pub location_description: String,
    #[serde(rename = "newConstruction")]
    pub new_construction: bool,
    #[serde(rename = "projectId")]
    pub project_id: Option<String>,
    #[serde(rename = "publishedAt")]
    pub published_at: String,
    #[serde(rename = "recordType")]
    pub record_type: String,
    #[serde(rename = "removedBeforeShowing")]
    pub removed_before_showing: bool,
    pub rooms: Option<String>,
    pub saved: bool,
    pub showings: Vec<String>,
    pub slug: String,
    #[serde(rename = "squareMeterPrice")]
    pub square_meter_price: Option<String>,
    #[serde(rename = "streetAddress")]
    pub street_address: String,
    #[serde(rename = "thumbnails({\"format\":\"ITEMGALLERY_CUT\"})")]
    pub thumbnails: Vec<String>,
    pub upcoming: bool,
}

#[derive(Debug, Serialize)]
pub struct CsvRow {
    pub id: String,
    pub listing_id: String,
    pub street_address: String,
    pub sold_at: String,
    pub sold_at_label: String,
    pub asking_price: Option<i64>,
    pub final_price: Option<i64>,
    pub living_area: Option<f64>,
    pub location: Option<String>,
    pub location_description: String,
    pub fee: Option<i64>,
    pub square_meter_price: Option<i64>,
    pub rooms: String,
    pub price_change: Option<f64>,
    pub broker_agency_name: String,
    pub broker_name: Option<String>,
    pub labels: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SaleCard {
    pub id: String,
    #[serde(rename = "listingId")]
    pub listing_id: String,
    pub slug: String,
    #[serde(rename = "streetAddress")]
    pub street_address: String,
    #[serde(rename = "soldAt")]
    pub sold_at: String,
    #[serde(rename = "soldAtLabel")]
    pub sold_at_label: String,
    #[serde(rename = "askingPrice")]
    pub asking_price: String,
    #[serde(rename = "finalPrice")]
    pub final_price: String,
    #[serde(rename = "livingArea")]
    pub living_area: String,
    #[serde(rename = "locationDescription")]
    pub location_description: String,
    pub fee: Option<String>,
    #[serde(rename = "squareMeterPrice")]
    pub square_meter_price: String,
    #[serde(rename = "housingForm")]
    pub housing_form: HousingForm,
    pub rooms: Option<String>,
    #[serde(rename = "landArea")]
    pub land_area: Option<String>,
    #[serde(rename = "priceChange")]
    pub price_change: Option<String>,
    pub coordinates: Coordinates,
    #[serde(rename = "brokerAgencyName")]
    pub broker_agency_name: String,
    #[serde(rename = "brokerAgencyThumbnail")]
    pub broker_agency_thumbnail: Option<String>,
    #[serde(rename = "brokerThumbnail")]
    pub broker_thumbnail: Option<String>,
    #[serde(rename = "brokerName")]
    pub broker_name: Option<String>,
    pub labels: Vec<Label>,
    pub product: String,
    #[serde(rename = "recordType")]
    pub record_type: String,
}

impl SaleCard {
    pub fn to_csv_row(&self, location: Option<&str>) -> anyhow::Result<CsvRow> {
        let clean_price = |price: &String| -> Option<i64> {
            if price.contains("saknas") {
                return None;
            }

            let cleaned = price
                .replace("kr", "")
                .replace("/mån", "")
                .replace(['\u{00A0}', ' '], "");

            cleaned.parse::<i64>().ok()
        };

        let clean_percentage = |pct: &String| -> Option<f64> {
            if pct.contains("±0") {
                return Some(0.0);
            }

            Some(
                pct.replace('%', "")
                    .replace(',', ".")
                    .replace('\u{00A0}', "")
                    .parse::<f64>()
                    .unwrap_or(0.0),
            )
        };

        let living_area = self
            .living_area
            .replace("m²", "")
            .replace(',', ".")
            .replace([' ', '\u{00A0}'], "")
            .parse::<f64>()
            .ok();

        let label_str = self
            .labels
            .iter()
            .filter_map(|label| {
                label
                    .text
                    .as_ref()
                    .map(|text| format!("{}:{}", label.category, text))
            })
            .collect::<Vec<_>>()
            .join(";");

        let square_meter_price_cleaned = self.square_meter_price.replace("kr/m²", "");

        Ok(CsvRow {
            id: self.id.clone(),
            listing_id: self.listing_id.clone(),
            street_address: self.street_address.clone(),
            sold_at: self.sold_at.clone(),
            sold_at_label: self.sold_at_label.clone(),
            asking_price: clean_price(&self.asking_price),
            final_price: clean_price(&self.final_price),
            living_area,
            location: location.map(|s| s.to_string()),
            location_description: self.location_description.clone(),
            fee: self.fee.as_ref().and_then(clean_price),
            square_meter_price: clean_price(&square_meter_price_cleaned),
            rooms: self.rooms.clone().unwrap_or_default(),
            price_change: self.price_change.as_ref().and_then(clean_percentage),
            broker_agency_name: self.broker_agency_name.clone(),
            broker_name: self.broker_name.clone(),
            labels: label_str,
            url: format!("https://www.hemnet.se/salda/{}", self.slug),
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApolloState {
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageProps {
    #[serde(rename = "__APOLLO_STATE__")]
    pub apollo_state: ApolloState,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HemnetListingsResponse {
    #[serde(rename = "pageProps")]
    pub page_props: PageProps,
}
