use std::env;
use std::fs::File;
use std::sync::Arc;

use crate::authorize::{ApplicationCredentials, TokenManager, TLS_CERTS};
use tokio::sync::Mutex;
use tonic::transport::{Certificate, Channel, ClientTlsConfig};
use tonic::{IntoRequest, Request};
// use crate::container::api;
use crate::container::api::cluster_manager_client::ClusterManagerClient;
use crate::container::api::{ ListClustersRequest,Cluster,GetServerConfigRequest,ServerConfig};
use crate::container::Error;

/// The Cloud Storage client, tied to a specific project.
#[derive(Clone)]
pub struct Client {
    pub(crate) project_name: String,
    pub(crate) service: ClusterManagerClient<Channel>,
    pub(crate) token_manager: Arc<Mutex<TokenManager>>,
}

impl Client {
    #[allow(unused)]
    pub(crate) const DOMAIN_NAME: &'static str = "container.googleapis.com";
    pub(crate) const ENDPOINT: &'static str = "https://container.googleapis.com";
    pub(crate) const SCOPES: [&'static str; 1] = [
        "https://www.googleapis.com/auth/cloud-platform",
    ];
    #[allow(dead_code)]
    pub(crate) fn uri(uri: &str) -> String {
        if uri.starts_with('/') {
            format!("{}{}", Client::ENDPOINT, uri)
        } else {
            format!("{}/{}", Client::ENDPOINT, uri)
        }
    }

    pub(crate) async fn construct_request<T: IntoRequest<T>>(
        &mut self,
        request: T,
    ) -> Result<Request<T>, Error> {
        let mut request = request.into_request();
        let token = self.token_manager.lock().await.token().await?;
        let metadata = request.metadata_mut();
        metadata.insert("authorization", token.parse().unwrap());
        Ok(request)
    }

    /// Create a new client for the specified project.
    ///
    /// Credentials are looked up in the `GOOGLE_APPLICATION_CREDENTIALS` environment variable.
    pub async fn new(project_name: impl Into<String>) -> Result<Client, Error> {
        let path = env::var("GOOGLE_APPLICATION_CREDENTIALS")?;
        let file = File::open(path)?;
        let creds = json::from_reader(file)?;

        Client::from_credentials(project_name, creds).await
    }

    /// Create a new client for the specified project with custom credentials.
    pub async fn from_credentials(
        project_name: impl Into<String>,
        creds: ApplicationCredentials,
    ) -> Result<Client, Error> {
        let tls_config = ClientTlsConfig::new()
            .ca_certificate(Certificate::from_pem(TLS_CERTS))
            .domain_name(Client::DOMAIN_NAME);

        let channel = Channel::from_static(Client::ENDPOINT)
            .tls_config(tls_config)?
            .connect()
            .await?;

        Ok(Client {
            project_name: project_name.into(),
            service: ClusterManagerClient::new(channel),
            token_manager: Arc::new(Mutex::new(TokenManager::new(
                creds,
                Client::SCOPES.as_ref(),
            ))),
        })
    }
 
    pub async fn list_clusters(&mut self) -> Result<Vec<Cluster>, Error> {

        let request = ListClustersRequest {
            parent: format!(
                "projects/{0}/locations/{1}/clusters",
                self.project_name,
                "australia-southeast1"
            ),
            project_id: self.project_name.clone(),
            zone: "-".to_string(),
        };
        let request = self.construct_request(request).await?;
        let response = self.service.list_clusters(request).await?;
        let response = response.into_inner(); // find out what it does
        // let m: Vec<String> = response
        // .clusters
        // .into_iter()
        // .map(|cluster| cluster.name).collect();
        let m: Vec<Cluster> = response.clusters;
        Ok(m)
    }

    pub async fn get_server_config(&mut self, zone: String) -> Result<ServerConfig,Error> {
        let request = GetServerConfigRequest {
            name: format!(
                "projects/{0}/locations/{1}/serverConfig",
                self.project_name,
                "australia-southeast1"
            ),
            project_id: self.project_name.clone(),
            zone: zone,
        };
        let request = self.construct_request(request).await?;
        let response = self.service.get_server_config(request).await?;
        Ok(response.into_inner())
    }

}
