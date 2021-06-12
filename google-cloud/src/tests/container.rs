use std::io::{self, Write};

use serde::{Deserialize, Serialize};

use crate::container;

macro_rules! assert_ok {
    ($expr:expr) => {
        match $expr {
            Ok(value) => {
               
                value
            }
                ,
            Err(err) => {
                panic!("asserted result is an error: {}", err);
            }
        }
    };
}

macro_rules! assert_some {
    ($expr:expr) => {
        match $expr {
            Some(value) => value,
            None => {
                panic!("asserted option is an none");
            }
        }
    };
}

async fn setup_client() -> Result<container::Client, container::Error> {
    println!("I am from match");
    let creds = super::load_creds();
    container::Client::from_credentials(env!("GCP_TEST_PROJECT"), creds).await
}

#[tokio::test]
async fn list_k8_clusters() {
    //? Setup test client.

    let mut client = assert_ok!(setup_client().await);
    let x = assert_ok!(client.list_clusters().await);
    println!("final {:?}",x);
    
    
}
