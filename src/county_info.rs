use hyper::Uri;
use serde::{Deserialize, Serialize};
#[derive(Debug)]
pub enum ConversionError {
    MissingField(String),
    InvalidField(String),
}

impl std::fmt::Display for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingField(s) => write!(f, "Listing is missing field {}", s)?,
            Self::InvalidField(s) => write!(f, "Invalid field datatype for {}", s)?
        }
        Ok(())
    }
}
impl std::error::Error for ConversionError {}


pub struct Coordinate {
    pub latitude: f64,
    pub longitude: f64,
}
pub struct LandListing {
    pub acres: f32,
    pub price: f64,
    pub address: String,
    pub city: String,
    pub zip: String,
    pub description: String,
    pub seller: String,
    pub coordinate: Option<Coordinate>,
}
pub trait RealEstateQuery {
    type Listing: TryInto<LandListing, Error=Box<dyn std::error::Error>>;
    fn query(&self) -> &dyn RealEstateResults<Listing=Self::Listing>;
    fn url(&self) -> String;
}

pub trait RealEstateResults {
    type Listing: TryInto<LandListing, Error=Box<dyn std::error::Error>>;
    fn backend_listings(&self) -> &Vec<Self::Listing>;

    fn listings(&self) -> Result<Vec<LandListing>, Box<dyn std::error::Error>>
    where
        Self::Listing: Clone
    {
        let mut listing_list = Vec::new();
        for l in self.backend_listings().iter() {
            listing_list.push(l.clone().try_into()?)
        }
        Ok(listing_list)
    }
}
