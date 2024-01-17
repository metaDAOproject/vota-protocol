use thiserror::Error;

#[derive(Error, Debug)]
pub enum AccountGenError {
    #[error("Invalid current working directory. Run script from project root directory")]
    InvalidCwd,
    #[error("Invalid account data")]
    InvalidAccountData,
}
