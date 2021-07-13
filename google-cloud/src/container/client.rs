use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::sync::Arc;

use crate::authorize::{ApplicationCredentials, TokenManager, TLS_CERTS};
use tokio::sync::Mutex;
use tonic::transport::{Certificate, Channel, ClientTlsConfig};
use tonic::{IntoRequest, Request};
use crate::container::GCluster;
// use crate::container::api;
use crate::container::api::cluster_manager_client::ClusterManagerClient;
use crate::container::api::{
    Cluster, CreateClusterRequest, GetClusterRequest, GetJsonWebKeysRequest,
    GetJsonWebKeysResponse, GetNodePoolRequest, GetOperationRequest, GetServerConfigRequest,
    ListClustersRequest, ListClustersResponse,
    ListNodePoolsRequest, ListNodePoolsResponse, ListOperationsRequest, ListOperationsResponse,
    ListUsableSubnetworksRequest, ListUsableSubnetworksResponse,NodePool,Operation, ServerConfig,
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

    ///CreateCluster creates a cluster, consisting of the specified number and type of Google Compute Engine instances.
    ////By default, the cluster is created in the project’s default network (at https://cloud.google.com/compute/docs/networks-and-firewalls#networks).
    ///One firewall is added for the cluster. After cluster creation, the Kubelet creates routes for each node to allow the containers on that node to
    ////communicate with all other instances in the cluster.
    /// Finally, an entry is added to the project’s global metadata indicating which CIDR range the cluster is using.
    #[allow(deprecated)]
    pub async fn create_cluster(
        &mut self,
        cluster: Option<GCluster>,
        location: &str,
        zone: &str
    ) -> Result<Operation, Error> {
        let r_cluster = match cluster {
            Some(Cluster) => Some(Cluster.into()),
            _ => None,
        };
        let request = CreateClusterRequest {
            project_id: self.project_name.clone(),
            zone: zone.to_owned(),
            cluster: r_cluster,
            parent: format!(
                "projects/{0}/locations/{1}",
                self.project_name.clone(),
                location
            ),
        };
        let request = self.construct_request(request).await?;
        let response = self.service.create_cluster(request).await?;
        let response = response.into_inner();
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
        let response = response.into_inner();
        Ok(response)
    }

    /// GetJSONWebKeys gets the public component of the cluster signing keys in JSON Web Key format.
    /// This API is not yet intended for general use, and is not available for all clusters.
    #[allow(deprecated)]
    pub async fn get_json_web_keys(
        &mut self,
        cluster_id: &str,
        location: &str,
    ) -> Result<GetJsonWebKeysResponse, Error> {
        let request = GetJsonWebKeysRequest {
            parent: format!(
                "projects/{0}/locations/{1}/clusters/{2}",
                self.project_name, location, cluster_id
            ),
        };
        let request = self.construct_request(request).await?;
        let response = self.service.get_json_web_keys(request).await?;
        let response = response.into_inner();
        Ok(response)
    }

    /// GetNodePool retrieves the requested node pool.
    #[allow(deprecated)]
    pub async fn get_node_pool(
        &mut self,
        cluster_id: &str,
        location: &str,
        node_pool_id: &str,
        zone: &str,
    ) -> Result<NodePool, Error> {
        let request = GetNodePoolRequest {
            project_id: self.project_name.clone(),
            zone: zone.to_owned(),
            cluster_id: cluster_id.to_owned(),
            node_pool_id: node_pool_id.to_owned(),
            name: format!(
                "projects/{0}/locations/{1}/clusters/{2}/nodePools/{3}",
                self.project_name, location, cluster_id, node_pool_id
            ),
        };
        let request = self.construct_request(request).await?;
        let response = self.service.get_node_pool(request).await?;
        let response = response.into_inner();
        Ok(response)
    }

    /// GetOperation gets the specified operation..
    #[allow(deprecated)]
    pub async fn get_operation(
        &mut self,
        location: &str,
        operation_id: &str,
        zone: &str,
    ) -> Result<Operation, Error> {
        let request = GetOperationRequest {
            project_id: self.project_name.clone(),
            zone: zone.to_owned(),
            operation_id: operation_id.to_owned(),
            name: format!(
                "projects/{0}/locations/{1}/operations/{2}`",
                self.project_name, location, operation_id
            ),
        };
        let request = self.construct_request(request).await?;
        let response = self.service.get_operation(request).await?;
        let response = response.into_inner();
        Ok(response)
    }
    ///GetServerConfig returns configuration info about the Google Kubernetes Engine service.
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
    ///ListClusters lists all clusters owned by a project in either the specified zone or all zones.
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
        let response = response.into_inner();
        Ok(response)
    }
    ///ListNodePools lists the node pools for a cluster.
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
        let response = response.into_inner();
        Ok(response)
    }
    ///ListOperations lists all operations in a project in a specific zone or all zones.
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
        let response = response.into_inner();
        Ok(response)
    }
    ///ListUsableSubnetworks lists subnetworks that are usable for creating clusters in a project.
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
        let response = response.into_inner();
        Ok(response)
    }
}
