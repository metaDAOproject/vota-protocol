use anchor_lang::prelude::*;

#[error_code]
#[derive(PartialEq)]
pub enum ErrorCode {
    #[msg("Cannot modify completed epochs")]
    CompletedEpoch,
    #[msg("Allocated vote amount is greater than total vote amount")]
    InvalidAllocatedVoteAmount,
}
