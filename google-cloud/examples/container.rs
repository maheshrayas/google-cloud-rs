
use google_cloud::container::{Client};
use google_cloud::error::Error;
use google_cloud::authorize::ApplicationCredentials;

struct GcpConnect {
   conn: ApplicationCredentials
}

impl GcpConnect {

    async fn new() -> Self {
        let creds = std::env::var("GOOGLE_APPLICATION_CREDENTIALS")
    .expect("env GOOGLE_APPLICATION_CREDENTIALS not set");

    let apps_creds = json::from_str::<ApplicationCredentials>(&creds)
    .expect("incorrect application credentials format");
 
    GcpConnect{
        conn: apps_creds
    }

    }

}

#[tokio::main]
pub async fn main() -> Result<(),Error> {
    let  g_con = GcpConnect::new().await;
    let mut x =  Client::from_credentials(env!("GCP_TEST_PROJECT"), g_con.conn).await?;
    list_cluster(&mut x).await?;
    get_server_config(&mut x).await?;
    Ok(())
}

async fn list_cluster(c:&mut Client) -> Result<(),Error> {
    let m = c.list_clusters().await?;
    println!("hi {:?}",m);
    Ok(())
}

async fn get_server_config(c:&mut Client) -> Result<(),Error> {
    let zone = String::from("australia-southeast1-a");
    let m = c.get_server_config(zone).await?;
    println!("hi {:?}",m);
    Ok(())
}
