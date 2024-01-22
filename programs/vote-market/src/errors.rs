use anchor_lang::prelude::*;

#[error_code]
#[derive(PartialEq)]
pub enum ErrorCode {
    #[msg("Cannot modify completed epochs")]
    CompletedEpoch,
    #[msg("Epoch voting not completed")]
    EpochVotingNotCompleted,
    #[msg("Allocated vote amount is greater than total vote amount")]
    InvalidAllocatedVoteAmount,
    #[msg("Epoch overflow")]
    EpochOverflow,
    #[msg("Invalid vote payment mint")]
    InvalidMint,
    #[msg("The initial buyer is the only reward receiver for this epoch")]
    InvalidBuyer,
    #[msg("Unable to calcualate vote power")]
    InvalidVotePower,
}
