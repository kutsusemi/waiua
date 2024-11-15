#[derive(thiserror::Error, Debug)]
pub enum DomainError {
    #[error("Infra error: {0}")]
    InfraError(String),
}
