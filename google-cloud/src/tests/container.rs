use crate::container;

macro_rules! assert_ok {
    ($expr:expr) => {
        match $expr {
            Ok(value) => value,
            Err(err) => {
                panic!("asserted result is an error: {}", err);
            }
        }
    };
}

// macro_rules! assert_some {
//     ($expr:expr) => {
//         match $expr {
//             Some(value) => value,
//             None => {
//                 panic!("asserted option is an none");
//             }
//         }
//     };
// }

async fn setup_client() -> Result<container::Client, container::Error> {
    let creds = super::load_creds();
    container::Client::from_credentials(env!("GCP_TEST_PROJECT"), creds).await
}

#[tokio::test]
async fn list_k8_clusters() {
    //? Setup test client.
    let mut client = assert_ok!(setup_client().await);
    assert_ok!(client.list_clusters("australia-southeast1").await);
}

#[tokio::test]
async fn get_server_config() {
    //? Setup test client.
    let mut client = assert_ok!(setup_client().await);
    let zone = String::from("australia-southeast1-a");
    assert_ok!(client.get_server_config(zone).await);
}

#[tokio::test]
async fn list_node_pools() {
    //? Setup test client.
    let mut client = assert_ok!(setup_client().await);
    let location = "australia-southeast1";
    let cluster_id = "cluster-1";
    assert_ok!(client.list_node_pools(cluster_id, location).await);
}

#[tokio::test]
async fn list_operations() {
    //? Setup test client.
    let mut client = assert_ok!(setup_client().await);
    let location = "australia-southeast1";
    let zone = "australia-southeast1-a";
    assert_ok!(client.list_operations(location,zone).await);
}

#[tokio::test]
async fn list_usable_subnetworks() {
    //? Setup test client.
    let mut client = assert_ok!(setup_client().await);
    let filter = "networkProjectId=admin-project-307508";
    let page_size: i32 = 12;
    let page_token = "";
    assert_ok!(client.list_usable_subnetworks(filter, page_size, page_token).await);
}

#[tokio::test]
async fn get_cluster() {
    //? Setup test client.
    let mut client = assert_ok!(setup_client().await);
    let cluster_id = "cluster-1";
    let zone = "australia-southeast1-a";
    let location = "australia-southeast1";
    assert_ok!(client.get_cluster(cluster_id, location, zone).await);
}