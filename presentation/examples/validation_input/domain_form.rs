use crate::domain::{Date, PostalCode};
use crate::domain_error::DomainError;

#[derive(Default, Clone)]
pub struct DomainFormOpt {
    pub date: Option<Date>,
    pub postal_code: Option<PostalCode>,
}

#[derive(Debug)]
pub struct DomainForm {
    pub date: Date,
    pub postal_code: PostalCode,
}

impl TryFrom<DomainFormOpt> for DomainForm {
    type Error = DomainError;
    fn try_from(opt: DomainFormOpt) -> Result<Self, Self::Error> {
        Ok(Self {
            date: opt.date.ok_or(DomainError::ValidateFormError)?,
            postal_code: opt.postal_code.ok_or(DomainError::ValidateFormError)?,
        })
    }
}
