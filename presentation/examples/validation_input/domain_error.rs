#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum DomainError {
    #[error("DomainParseError")]
    DomainParseError,

    #[error("ValidateFormError")]
    ValidateFormError,
}
