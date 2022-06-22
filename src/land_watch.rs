use std::str::Lines;

use serde::Deserialize;
use crate::county_info::{LandListing, ConversionError, Coordinate, RealEstateQuery, RealEstateResults};

struct LandWatchQuery {
    county: String,
    state: String
}

impl LandWatchQuery {
    fn new(county: String, state: String) -> Self {
        Self {
            county,
            state
        }
    }
}

impl RealEstateQuery for LandWatchQuery {
    type Listing = LandWatchLandListing;
    fn query(&self) ->  {

    }
}

#[derive(Deserialize, Debug)]
pub struct Results {
    searchResults: PropertyResults,
}

impl RealEstateResults for Results {
    type Listing = LandWatchLandListing;
    fn backend_listings(&self) -> &Vec<LandWatchLandListing> {
        &self.searchResults.propertyResults
    }
}
impl Results {
    pub fn number_results(&self) -> u32 {
        self.searchResults.totalCount
    }
}

#[derive(Deserialize, Debug)]
pub struct PropertyResults {
    propertyResults: Vec<LandWatchLandListing>,
    totalCount: u32
}

#[derive(Deserialize, Debug)]
pub struct LandWatchLandListing {
    acres: f32,
    address: String,
    auctionDate: String,
    brokerCompany: String,
    city: String,
    price: f64,
    zip: String,
    state: String,
    county: String,
    description: String,
    latitude: f64,
    longitude: f64
}

impl std::fmt::Display for LandWatchLandListing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}", self.acres, self.price, self.address, self.auctionDate, self.brokerCompany, self.city, self.zip, self.longitude, self.latitude)?;
        return Ok(())
    }
}

impl TryFrom<LandWatchLandListing> for LandListing {
    type Error = Box<ConversionError>;
    fn try_from(value: LandWatchLandListing) -> Result<Self, Self::Error> {
        Ok(LandListing {
            acres: value.acres,
            price: value.price,
            address: value.address,
            city: value.city,
            zip: value.zip,
            seller: value.brokerCompany,
            description: value.description,
            coordinate: Some(Coordinate {
                latitude: value.latitude,
                longitude: value.longitude
            })
        })
    }
}