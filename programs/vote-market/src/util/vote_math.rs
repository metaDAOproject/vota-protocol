use crate::errors::VoteMarketError;
use anchor_lang::err;

use anchor_lang::prelude::*;

pub fn get_user_payment(
    total_power: u64,
    allocated_power: u64,
    total_vote_payment: u64,
) -> Result<u64> {
    if total_power == 0 || allocated_power == 0 || total_vote_payment == 0 {
        return Ok(0);
    }
    if allocated_power > total_power {
        return err!(VoteMarketError::InvalidAllocatedVoteAmount);
    }
    ::u128::mul_div_u64(allocated_power, total_vote_payment, total_power)
        .ok_or(VoteMarketError::InvalidVotePower.into())
}

pub fn get_fee(payment_to_user: u64, fee: u16) -> Result<u64> {
    if fee > 10000 {
        return err!(VoteMarketError::InvalidFee);
    }
    ::u128::mul_div_u64(payment_to_user, fee as u64, 10000)
        .ok_or(VoteMarketError::InvalidVotePower.into())
}

// Unit tests
#[cfg(test)]
mod test_vote_math {
    use super::*;
    use anchor_lang::error::Error::AnchorError;

    #[test]
    fn test_get_fee() {
        assert_eq!(get_fee(100, 0), Ok(0));
        assert_eq!(get_fee(100, 10000), Ok(100));
        assert_eq!(get_fee(100, 5000), Ok(50));
        assert_eq!(get_fee(100, 600), Ok(6));
        assert_eq!(get_fee(101, 600), Ok(6));
        match get_fee(100, 10001) {
            Ok(_) => panic!("should have failed"),
            Err(_) => {}
        }
    }
    #[test]
    fn test_calculate_voter_share() {
        assert_eq!(get_user_payment(100, 0, 100), Ok(0));
        assert_eq!(get_user_payment(100, 100, 100), Ok(100));
        assert_eq!(get_user_payment(100, 50, 100), Ok(50));
        assert_eq!(get_user_payment(100, 25, 100), Ok(25));
        assert_eq!(get_user_payment(100, 75, 100), Ok(75));
        assert_eq!(get_user_payment(100, 100, 100), Ok(100));
        assert_eq!(get_user_payment(100, 0, 200), Ok(0));
        assert_eq!(get_user_payment(100, 100, 200), Ok(200));
        assert_eq!(get_user_payment(100, 50, 200), Ok(100));
        assert_eq!(get_user_payment(100, 25, 200), Ok(50));
        assert_eq!(get_user_payment(100, 75, 200), Ok(150));
        assert_eq!(get_user_payment(100, 100, 200), Ok(200));
        assert_eq!(get_user_payment(100, 0, 0), Ok(0));
        assert_eq!(get_user_payment(0, 100, 100), Ok(0));
        match get_user_payment(100, 101, 100) {
            Ok(_) => panic!("should have failed"),
            Err(_) => (),
        }
    }
}
