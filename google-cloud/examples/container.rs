use google_cloud::authorize::ApplicationCredentials;
use google_cloud::container::Client;
use google_cloud::error::Error;

struct GcpConnect {
    conn: ApplicationCredentials,
}

impl GcpConnect {
    async fn new() -> Self {
        let creds = std::env::var("GOOGLE_APPLICATION_CREDENTIALS")
            .expect("env GOOGLE_APPLICATION_CREDENTIALS not set");

        let apps_creds = json::from_str::<ApplicationCredentials>(&creds)
            .expect("incorrect application credentials format");

        GcpConnect { conn: apps_creds }
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    let g_con = GcpConnect::new().await;
    let mut x = Client::from_credentials(env!("GCP_TEST_PROJECT"), g_con.conn).await?;
    list_cluster(&mut x).await?;
    get_server_config(&mut x).await?;
    list_node_pools(&mut x).await?;
    list_operations(&mut x).await?;
    list_usable_subnetworks(&mut x).await?;
    get_cluster(&mut x).await?;
    Ok(())
}

async fn list_cluster(c: &mut Client) -> Result<(), Error> {
    let m = c.list_clusters("australia-southeast1").await?;
    //println!("{:?}", m.clusters.len());
    Ok(())
}

async fn get_server_config(c: &mut Client) -> Result<(), Error> {
    let zone = String::from("australia-southeast1-a");
    let m = c.get_server_config(zone).await?;
    //println!("hi {:?}",m);
    Ok(())
}

async fn list_node_pools(c: &mut Client) -> Result<(), Error> {
    let location = "australia-southeast1";
    let cluster_id = "cluster-1";
    let m = c.list_node_pools(cluster_id, location).await?;
    //println!("hi {:?}", m);
    Ok(())
}

async fn list_operations(c: &mut Client) -> Result<(), Error> {
    let location = "australia-southeast1";
    let zone = "australia-southeast1-a";
    let m = c.list_operations(location, zone).await?;
    // for x in m.operations.iter() {
    //     println!("name -> {}", x.name);
    // }
    //println!("name -> {:?}", m);
    Ok(())
}

async fn list_usable_subnetworks(c: &mut Client) -> Result<(), Error> {
    let filter = "networkProjectId=admin-project-307508";
    let page_size: i32 = 12;
    let page_token = "";
    let m = c
        .list_usable_subnetworks(filter, page_size, page_token)
        .await?;
    // for x in &m.subnetworks {
    //     println!("name -> {}", x.ip_cidr_range);
    // }
    // println!("name -> {:?}", m);
    Ok(())
}

async fn get_cluster(c: &mut Client) -> Result<(), Error> {
    let filter = "networkProjectId=admin-project-307508";
    let page_size: i32 = 12;
    let cluster_id = "cluster-1";
    let zone = "australia-southeast1-a";
    let location = "australia-southeast1";
    let m = c.get_cluster(cluster_id, location, zone).await?;
    println!("name -> {:?}", m);
    Ok(())
}
