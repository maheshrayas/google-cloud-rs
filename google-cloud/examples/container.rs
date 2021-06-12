
use google_cloud::container::Client;
use google_cloud::error::Error;
use google_cloud::authorize::ApplicationCredentials;


#[tokio::main]
pub async fn main() -> Result<(),Error> {
    let creds = std::env::var("GOOGLE_APPLICATION_CREDENTIALS")
    .expect("env GOOGLE_APPLICATION_CREDENTIALS not set");

    let apps_creds = json::from_str::<ApplicationCredentials>(&creds)
    .expect("incorrect application credentials format");
    connect(apps_creds).await?;
    Ok(())
}

async fn connect(creds:ApplicationCredentials) -> Result<(),Error> {
    println!("hellow workrkr");
    let mut x =Client::from_credentials(env!("GCP_TEST_PROJECT"), creds).await?;
    let m = x.list_clusters().await?;
    println!("hi {:?}",m);

    Ok(())
}

