mod client;
mod cluster;
mod api {
    include!("api/google.container.v1.rs");
}

pub use self::client::*;
pub use self::cluster::*;
// pub use self::api::Cluster;
// pub use self::api::NodePool;
pub use self::api::*;

/// The error type for the PubSub module.
pub type Error = crate::error::Error;
