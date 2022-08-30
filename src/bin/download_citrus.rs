use hyper::Client;
use land_scraper::county_info::ListingProvider;
use land_scraper::county_info::ServiceQuery;
use land_scraper::providers::land_watch::LandWatch;
use land_scraper::providers::land_watch::LandWatchLandListing;

#[tokio::main]
async fn main() {
    let https = hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .https_only()
        .enable_http1()
        .build();
    let client: Client<_, hyper::Body> = Client::builder().build(https);
    let query = ServiceQuery::builder()
        .state("florida".to_string())
        .county("citrus".to_string());
    println!("{:?}", query);
    let listings = LandWatch::load(&query, &client).await.unwrap();
    let listings: Vec<LandWatchLandListing> = listings;
    println!("{:?}", listings.len());
    println!("{:?}", listings[0]);
}
