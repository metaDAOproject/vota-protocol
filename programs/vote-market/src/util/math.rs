use anchor_lang::error;
use anchor_lang::error::Error;
use anchor_lang::prelude::ProgramError;
use crate::errors::ErrorCode;

pub fn calculate_voter_share(total_power: u64, allocated_power: u64, total_vote_payment: u64) -> Result<u64, Error> {
    if total_power == 0  || allocated_power == 0 || total_vote_payment == 0 {
        return Ok(0);
    }
    if allocated_power > total_power {
        return Err(error!(ErrorCode::InvalidAllocatedVoteAmount))
    }
    let shifted_allocated_power= ((allocated_power as u128) << 64) / (total_power as u128);
    Ok((((total_vote_payment as u128) * shifted_allocated_power) >> 64) as u64)
}

// Unit tests
#[cfg(test)]
mod test_calculate_voter_share {
    use super::*;
    #[test]
    fn test_calculate_voter_share() {
        assert_eq!(calculate_voter_share(100, 0, 100), Ok(0));
        assert_eq!(calculate_voter_share(100, 100, 100), Ok(100));
        assert_eq!(calculate_voter_share(100, 50, 100), Ok(50));
        assert_eq!(calculate_voter_share(100, 25, 100), Ok(25));
        assert_eq!(calculate_voter_share(100, 75, 100), Ok(75));
        assert_eq!(calculate_voter_share(100, 100, 100), Ok(100));
        assert_eq!(calculate_voter_share(100, 0, 200), Ok(0));
        assert_eq!(calculate_voter_share(100, 100, 200), Ok(200));
        assert_eq!(calculate_voter_share(100, 50, 200), Ok(100));
        assert_eq!(calculate_voter_share(100, 25, 200), Ok(50));
        assert_eq!(calculate_voter_share(100, 75, 200), Ok(150));
        assert_eq!(calculate_voter_share(100, 100, 200), Ok(200));
        assert_eq!(calculate_voter_share(100, 0, 0), Ok(0));
        assert_eq!(calculate_voter_share(0, 100, 100), Ok(0));
        match calculate_voter_share(100, 101, 100) {
            Ok(_) => panic!("should have failed"),
            Err(_) => (),
        }
    }
}