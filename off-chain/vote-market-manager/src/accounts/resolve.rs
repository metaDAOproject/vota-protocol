use solana_program::pubkey::Pubkey;
use crate::{LOCKER, LOCKER_PROGRAM};

pub fn get_escrow_address_for_owner(owner: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[b"Escrow".as_ref(), LOCKER.as_ref(), owner.as_ref()],
        &LOCKER_PROGRAM,
    ).0
}
