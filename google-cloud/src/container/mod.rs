mod client;
mod api {
    include!("api/google.container.v1.rs");
}

pub use self::client::*;
pub use self::api::Cluster;

/// The error type for the PubSub module.
pub type Error = crate::error::Error;
