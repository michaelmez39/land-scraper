use hyper::Client;
use hyper::Uri;
mod land_watch;
mod county_info;
use crate::land_watch::{Results, LandWatchLandListing as LandListing};
use tokio::io::AsyncWriteExt;
use tokio::fs::File;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let https = hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .https_only()
        .enable_http1()
        .build();
    let saved_results = File::create("columbia.csv").await?;
    let client: Client<_, hyper::Body> = Client::builder().build(https);
    let written = get_pages(&client, saved_results).await?;
    println!("saved {} listings!", written);
    Ok(())
}

async fn get_pages<T>(client: &Client<T, hyper::Body>, mut file: File) -> Result<usize, Box<dyn std::error::Error + Send + Sync>>
where T: hyper::client::connect::Connect + Clone + Send + Sync + 'static{
    let url =  "https://www.landwatch.com/api/property/search/1113/florida-land-for-sale/columbia-county/undeveloped-land";
    let uri = url.parse::<Uri>()?;
    let resp = client.get(uri).await?;
    let resp = hyper::body::to_bytes(resp).await?;
    let v: Results = serde_json::from_slice(&resp)?;
    file.write(&to_csv(v.listings()).await.as_bytes());
    let mut page = 1;
    let mut num_results = v.listings().len();
    while v.number_results() > num_results as u32{
        let uri = format!("{}/page-{}", url, page);
        let resp = client.get(uri.parse::<Uri>()?).await?;
        let resp_bytes = hyper::body::to_bytes(resp).await?;
        let data: Results = serde_json::from_slice(&resp_bytes)?;
        file.write(&to_csv(v.listings()).await.as_bytes()).await?;

        page += 1;
        num_results += data.listings().len();
    }

    Ok(num_results)
}

async fn to_csv(listings: &Vec<LandListing>) -> String {
    listings.iter().fold(String::new(), |acc, listing| {
        format!("{}\n{}", acc, listing)
    })
}