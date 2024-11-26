use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    UPnP(#[from] rupnp::Error),
}
