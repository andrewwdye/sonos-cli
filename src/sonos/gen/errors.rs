use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    UPnP(#[from] rupnp::Error),
    #[error("missing expected field: {0}")]
    MissingField(String),
    #[error("error parsing field {0}")]
    ParseError(String),
    #[error("service {0} not found")]
    ServiceNotFound(String),
}
