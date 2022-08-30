use county_info::{ProviderError, ServiceQuery, ListingProvider};
use async_trait::async_trait;
use hyper::Uri;

struct Zillow;

#[async_trait]
impl ListingProvider for Zillow {
    type Listing = ZillowListing;
    type Error = ProviderError;
    async fn load<T>(query: &ServiceQuery, client: &Client<T, hyper::Body>) -> Result<Vec<Self::Listing>, Self::Error>
    where T: hyper::client::connect::Connect + Clone + Send + Sync + 'static {
        
        let uri = Uri::builder()
            .scheme("https")
            .authority("zillow.com")
            .path_and_query("/");

    }
}

