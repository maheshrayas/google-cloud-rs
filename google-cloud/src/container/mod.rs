mod client;
mod api {
    include!("api/google.container.v1.rs");
}

pub use self::client::*;

/// The error type for the PubSub module.
pub type Error = crate::error::Error;
