// #![allow(dead_code)]
// #![allow(non_snake_case)]
use crate::county_info::{
    Coordinate, LandListing, ListingProvider, ServiceQuery
};
use crate::error::{ConversionError, ProviderError};
use serde::Deserialize;

use hyper::Client;
use hyper::Uri;
use async_trait::async_trait;

pub struct LandWatch;

#[async_trait]
impl ListingProvider for LandWatch {
    type Listing = LandWatchLandListing;
    type Error = ProviderError;
    // type Output = std::pin::Pin<Box<dyn Future<Output=Result<Vec<Self::Listing>, Self::Error>>>>;
    async fn load<T>(query: &ServiceQuery, client: &Client<T, hyper::Body>) -> Result<Vec<Self::Listing>, Self::Error>
    where
        T: hyper::client::connect::Connect + Clone + Send + Sync + 'static,
    {
        let url =  &format!("https://www.landwatch.com/api/property/search/1113/{}-land-for-sale/{}-county/undeveloped-land", query.state, query.county.as_ref().ok_or(ProviderError::Query)?);
        let uri = url.parse::<Uri>().map_err(|_| ProviderError::Query)?;
        println!("url: {}", url);
        let resp = client.get(uri).await?;
        let resp = hyper::body::to_bytes(resp).await?;
        let resp: Results = serde_json::from_slice(&resp)?;
        
        let num_listings = resp.len();
        let mut results = resp.listings();
        let mut page = 1;
        while num_listings > results.len() {
            println!("{} of {}", results.len(), num_listings);
            let url = format!("{}/page-{}", url, page);
            let uri = url.parse::<Uri>().map_err(|_| ProviderError::Query)?;
            let resp = client.get(uri).await?;
            let resp_bytes = hyper::body::to_bytes(resp).await?;
            let resp_res: Results = serde_json::from_slice(&resp_bytes)?;
            results.append(&mut resp_res.listings());
            page += 1;
        }
    
        Ok(results)
    }
}

#[derive(Deserialize, Debug)]
pub struct Results {
    searchResults: PropertyResults,
}

impl Results {
    pub fn len(&self) -> usize {
        self.searchResults.totalCount as usize
    }
    pub fn listings(self) -> Vec<LandWatchLandListing> {
        self.searchResults.propertyResults
    }
}

#[derive(Deserialize, Debug)]
pub struct PropertyResults {
    propertyResults: Vec<LandWatchLandListing>,
    totalCount: u32,
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
    longitude: f64,
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
                longitude: value.longitude,
            }),
        })
    }
}
