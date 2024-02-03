use std::any::Any;
use std::fmt::{Debug, Display};
use anchor_lang::error_code;
use serde::de::StdError;

#[derive(Debug)]
pub enum VoteMarketManagerError {
    AddressNotFound,
}

impl std::error::Error for VoteMarketManagerError { }
impl Display for VoteMarketManagerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VoteMarketManagerError::AddressNotFound => write!(f, "Address not found"),
        }
    }
}

