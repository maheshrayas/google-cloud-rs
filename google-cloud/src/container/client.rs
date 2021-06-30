use std::env;
use std::fs::File;
use std::sync::Arc;

use crate::authorize::{ApplicationCredentials, TokenManager, TLS_CERTS};
use tokio::sync::Mutex;
use tonic::transport::{Certificate, Channel, ClientTlsConfig};
use tonic::{IntoRequest, Request};
// use crate::container::api;
use crate::container::api::cluster_manager_client::ClusterManagerClient;
use crate::container::api::{
    Cluster, GetClusterRequest, GetServerConfigRequest, ListClustersRequest, ListClustersResponse,
    ListNodePoolsRequest, ListNodePoolsResponse, ListOperationsRequest, ListOperationsResponse,
    ListUsableSubnetworksRequest, ListUsableSubnetworksResponse, ServerConfig,
};
use crate::container::Error;

/// The  client, tied to a specific project.
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
    pub(crate) const SCOPES: [&'static str; 1] = ["https://www.googleapis.com/auth/cloud-platform"];
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

    ///
    #[allow(deprecated)]
    pub async fn get_server_config(&mut self, zone: String) -> Result<ServerConfig, Error> {
        let request = GetServerConfigRequest {
            name: format!(
                "projects/{0}/locations/{1}/serverConfig",
                self.project_name, "australia-southeast1"
            ),
            project_id: self.project_name.clone(),
            zone: zone,
        };
        let request = self.construct_request(request).await?;
        let response = self.service.get_server_config(request).await?;
        Ok(response.into_inner())
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
    ///
    #[allow(deprecated)]
    pub async fn list_clusters(&mut self, location: &str) -> Result<ListClustersResponse, Error> {
        let request = ListClustersRequest {
            parent: format!(
                "projects/{0}/locations/{1}/clusters",
                self.project_name, location
            ),
            project_id: self.project_name.clone(),
            zone: "-".to_string(),
        };
        let request = self.construct_request(request).await?;
        let response = self.service.list_clusters(request).await?;
        let response = response.into_inner(); // find out what it does
        Ok(response)
    }
    ///
    #[allow(deprecated)]
    pub async fn list_node_pools(
        &mut self,
        cluster_id: &str,
        location: &str,
    ) -> Result<ListNodePoolsResponse, Error> {
        let request = ListNodePoolsRequest {
            parent: format!(
                "projects/{0}/locations/{1}/clusters/{2}",
                self.project_name, location, cluster_id
            ),
            cluster_id: cluster_id.to_string(),
            project_id: self.project_name.clone(),
            zone: "australia-southeast1-a".to_string(),
        };
        let request = self.construct_request(request).await?;
        let response = self.service.list_node_pools(request).await?;
        let response = response.into_inner(); // find out what it does
        Ok(response)
    }
    ///
    #[allow(deprecated)]
    pub async fn list_operations(
        &mut self,
        location: &str,
        zone: &str,
    ) -> Result<ListOperationsResponse, Error> {
        let request = ListOperationsRequest {
            parent: format!("projects/{0}/locations/{1}/", self.project_name, location,),
            project_id: self.project_name.clone(),
            zone: zone.to_owned(),
        };
        let request = self.construct_request(request).await?;
        let response = self.service.list_operations(request).await?;
        let response = response.into_inner(); // find out what it does
        Ok(response)
    }
    ///
    #[allow(deprecated)]
    pub async fn list_usable_subnetworks(
        &mut self,
        filter: &str,
        page_size: i32,
        page_token: &str,
    ) -> Result<ListUsableSubnetworksResponse, Error> {
        let request = ListUsableSubnetworksRequest {
            parent: format!("projects/{0}", self.project_name,),
            filter: filter.to_owned(),
            page_size: page_size,
            page_token: page_token.to_owned(),
        };
        let request = self.construct_request(request).await?;
        let response = self.service.list_usable_subnetworks(request).await?;
        let response = response.into_inner(); // find out what it does
        Ok(response)
    }

    ///Gets the details of a specific cluster.
    #[allow(deprecated)]
    pub async fn get_cluster(
        &mut self,
        cluster_id: &str,
        location: &str,
        zone: &str,
    ) -> Result<Cluster, Error> {
        let request = GetClusterRequest {
            name: format!(
                "projects/{0}/locations/{1}/clusters/{2}",
                self.project_name, location, cluster_id
            ),
            cluster_id: cluster_id.to_owned(),
            project_id: self.project_name.clone(),
            zone: zone.to_owned(),
        };
        let request = self.construct_request(request).await?;
        let response = self.service.get_cluster(request).await?;
        let response = response.into_inner(); // find out what it does
        Ok(response)
    }
}
