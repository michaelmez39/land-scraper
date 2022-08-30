use crate::error::{ConversionError, ProviderError};
use async_trait::async_trait;
use hyper::Client;
#[derive(Debug)]
pub struct ServiceQuery {
    pub state: String,
    pub county: Option<String>,
    pub city: Option<String>,
}

impl ServiceQuery {
    pub fn builder() -> Self {
        Self {
            county: None,
            state: String::new(),
            city: None,
        }
    }

    pub fn city(mut self, c: String) -> Self {
        self.city = Some(c);
        self
    }
    pub fn county(mut self, c: String) -> Self {
        self.county = Some(c);
        self
    }
    pub fn state(mut self, c: String) -> Self {
        self.state = c;
        self
    }
}
#[derive(Debug)]
pub struct Coordinate {
    pub latitude: f64,
    pub longitude: f64,
}
#[derive(Debug)]
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
pub struct ProviderListing<T> {
    listings: Vec<T>,
}
impl<T> ProviderListing<T> {
    pub fn new(listings: Vec<T>) -> Self {
        Self { listings }
    }
    pub fn raw_listings(self) -> Vec<T> {
        self.listings
    }
}

// impl<T> From<ProviderListing<T>> for Vec<T> {
//     fn from(pl: ProviderListing<T>) -> Vec<T> {
//         pl.listings
//     }
// }

impl<T> TryFrom<ProviderListing<T>> for Vec<LandListing>
where
    T: TryInto<LandListing, Error = Box<ConversionError>>,
{
    type Error = Box<ConversionError>;
    fn try_from(pl: ProviderListing<T>) -> Result<Vec<LandListing>, Self::Error> {
        pl.listings.into_iter().map(|l| l.try_into()).collect()
    }
}

// pub trait RealEstateQuery {
//     type Listing: TryInto<LandListing, Error=Box<ConversionError>>;
//     fn query(&self) -> Box<dyn RealEstateResults<Listing=Self::Listing>>;
//     fn url(&self) -> String;
// }

#[async_trait]
pub trait ListingProvider {
    type Error: Into<ProviderError>;
    type Listing: TryInto<LandListing>;
    async fn load<T>(
        query: &ServiceQuery,
        client: &Client<T, hyper::Body>,
    ) -> Result<Vec<Self::Listing>, Self::Error>
    where
        T: hyper::client::connect::Connect + Clone + Send + Sync + 'static;
}
