

use std::{collections::HashMap, convert::TryFrom};
use prost_types::Timestamp;
use crate::container::*;

#[derive(Default)]
pub struct GCluster {
    pub name: String,
    pub description: String,
    pub master_auth: Option<api::MasterAuth>,
    pub logging_service : String,
    pub monitoring_service: String,
    pub network: String,
    pub cluster_ipv4_cidr: String,
    pub addons_config:Option<api::AddonsConfig>,
    pub subnetwork: String,
    pub node_pools: Vec<api::NodePool>,
    pub locations: Vec<String>,
    pub enable_kubernetes_alpha: bool,
    pub resource_labels: HashMap<String,String>,
    pub label_fingerprint: String,
    pub legacy_abac: Option<api::LegacyAbac>,
    pub network_policy:Option<api::NetworkPolicy>,
    pub ip_allocation_policy: Option<api::IpAllocationPolicy>,
    pub master_authorized_networks_config: Option<api::MasterAuthorizedNetworksConfig>,
    pub maintenance_policy:Option<api::MaintenancePolicy>,
    pub binary_authorization:Option<api::BinaryAuthorization>,
    pub network_config:Option<api::NetworkConfig>,
    pub default_max_pods_constraint:Option<api::MaxPodsConstraint>,
    pub resource_usage_export_config:Option<api::ResourceUsageExportConfig>,
    pub authenticator_groups_config:Option<api::AuthenticatorGroupsConfig>,
    pub private_cluster_config : Option<api::PrivateClusterConfig>,
    pub database_encryption : Option<api::DatabaseEncryption>,
    pub vertical_pod_autoscaling: Option<api::VerticalPodAutoscaling>,
    pub shielded_nodes : Option<api::ShieldedNodes>,
    pub release_channel : Option<api::ReleaseChannel>,
    pub workload_identity_config : Option<api::WorkloadIdentityConfig>,
    pub self_link: String,
    pub endpoint: String,
    pub initial_cluster_version: String, //latest
    pub create_time: String,     //OP
    pub status : i32,        //OP
    pub node_ipv4_cidr_size: i32, //OP
    pub services_ipv4_cidr: String, //OP
    pub expire_time: String, //OP
    pub location: String,
    pub enable_tpu: bool,
    pub tpu_ipv4_cidr_block: String,
    pub conditions : Vec<api::StatusCondition>,
}

impl From<GCluster> for api::Cluster {
    fn from(c: GCluster) -> Self {
        api::Cluster {
            name: c.name,
            description: c.description,
            master_auth: c.master_auth,
            logging_service : c.logging_service,
            monitoring_service: c.monitoring_service,
            network: c.network,
            cluster_ipv4_cidr: c.cluster_ipv4_cidr,
            addons_config:c.addons_config,
            subnetwork: c.subnetwork,
            node_pools: c.node_pools,
            locations: c.locations,
            enable_kubernetes_alpha: c.enable_kubernetes_alpha,
            resource_labels: c.resource_labels,
            label_fingerprint: c.label_fingerprint,
            legacy_abac: c.legacy_abac,
            network_policy:c.network_policy,
            ip_allocation_policy: c.ip_allocation_policy,
            master_authorized_networks_config: c.master_authorized_networks_config,
            maintenance_policy:c.maintenance_policy,
            binary_authorization:c.binary_authorization,
            network_config:c.network_config,
            default_max_pods_constraint: c.default_max_pods_constraint,
            resource_usage_export_config: c.resource_usage_export_config,
            authenticator_groups_config:c.authenticator_groups_config,
            private_cluster_config : c.private_cluster_config,
            database_encryption : c.database_encryption,
            vertical_pod_autoscaling: c.vertical_pod_autoscaling,
            shielded_nodes : c.shielded_nodes,
            release_channel : c.release_channel,
            workload_identity_config :c.workload_identity_config,
            self_link: c.self_link,
            endpoint: c.endpoint,
            initial_cluster_version: c.initial_cluster_version, //late
            create_time: c.create_time,     //OP
            status : c.status,        //OP
            node_ipv4_cidr_size: c.node_ipv4_cidr_size, //OP
            services_ipv4_cidr: c.services_ipv4_cidr, //OP
            expire_time: c.expire_time, //OP
            location: c.location,
            enable_tpu: c.enable_tpu,
            tpu_ipv4_cidr_block: c.tpu_ipv4_cidr_block,
            conditions :c.conditions,
            ..Default::default()
        }
    }
}

impl Cluster {
    pub fn new() -> Cluster{
        Default::default()
    }
}