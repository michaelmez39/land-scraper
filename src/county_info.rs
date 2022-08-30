use hyper::Client;
use async_trait::async_trait;
#[derive(Debug)]
pub enum ConversionError {
    MissingField(String),
    InvalidField(String),
}

impl std::fmt::Display for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingField(s) => write!(f, "Listing is missing field {}", s)?,
            Self::InvalidField(s) => write!(f, "Invalid field datatype for {}", s)?,
        }
        Ok(())
    }
}
#[derive(Debug)]
pub enum ProviderError {
    Conversion(serde_json::Error),
    Network(hyper::Error),
    Query
}

impl From<hyper::Error> for ProviderError {
    fn from(e: hyper::Error) -> Self {
        ProviderError::Network(e)
    }
}

impl From<serde_json::Error> for ProviderError {
    fn from(e: serde_json::Error) -> Self {
        ProviderError::Conversion(e)
    }
}
impl std::fmt::Display for ProviderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Conversion(s) => write!(f, "Conversion failed: {}", s)?,
            Self::Network(s) => write!(f, "Error getting data: {}", s)?,
            Self::Query => write!(f,"Query was invalid")?
        }
        Ok(())
    }
}

impl std::error::Error for ProviderError {}
impl std::error::Error for ConversionError {}

#[derive(Debug)]
pub struct ServiceQuery {
    pub county: String,
    pub state: String,
    pub city: Option<String>,
}

impl ServiceQuery {
    pub fn builder() -> Self {
        Self {
            county: String::new(),
            state: String::new(),
            city: None,
        }
    }

    pub fn city(mut self, c: String) -> Self {
        self.city = Some(c);
        self
    }
    pub fn county(mut self, c: String) -> Self {
        self.county = c;
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
    pub fn raw_listings(self) -> Vec<T>{
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
}
