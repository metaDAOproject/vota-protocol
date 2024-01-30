#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod errors {
    use anchor_lang::prelude::*;
    #[repr(u32)]
    pub enum VoteMarketError {
        CompletedEpoch,
        EpochVotingNotCompleted,
        InvalidAllocatedVoteAmount,
        EpochOverflow,
        InvalidMint,
        InvalidBuyer,
        InvalidVotePower,
        MaxVoteBuyAmountNotSet,
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for VoteMarketError {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for VoteMarketError {
        #[inline]
        fn eq(&self, other: &VoteMarketError) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for VoteMarketError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    VoteMarketError::CompletedEpoch => "CompletedEpoch",
                    VoteMarketError::EpochVotingNotCompleted => "EpochVotingNotCompleted",
                    VoteMarketError::InvalidAllocatedVoteAmount => {
                        "InvalidAllocatedVoteAmount"
                    }
                    VoteMarketError::EpochOverflow => "EpochOverflow",
                    VoteMarketError::InvalidMint => "InvalidMint",
                    VoteMarketError::InvalidBuyer => "InvalidBuyer",
                    VoteMarketError::InvalidVotePower => "InvalidVotePower",
                    VoteMarketError::MaxVoteBuyAmountNotSet => "MaxVoteBuyAmountNotSet",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for VoteMarketError {
        #[inline]
        fn clone(&self) -> VoteMarketError {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for VoteMarketError {}
    impl VoteMarketError {
        /// Gets the name of this [#enum_name].
        pub fn name(&self) -> String {
            match self {
                VoteMarketError::CompletedEpoch => "CompletedEpoch".to_string(),
                VoteMarketError::EpochVotingNotCompleted => {
                    "EpochVotingNotCompleted".to_string()
                }
                VoteMarketError::InvalidAllocatedVoteAmount => {
                    "InvalidAllocatedVoteAmount".to_string()
                }
                VoteMarketError::EpochOverflow => "EpochOverflow".to_string(),
                VoteMarketError::InvalidMint => "InvalidMint".to_string(),
                VoteMarketError::InvalidBuyer => "InvalidBuyer".to_string(),
                VoteMarketError::InvalidVotePower => "InvalidVotePower".to_string(),
                VoteMarketError::MaxVoteBuyAmountNotSet => {
                    "MaxVoteBuyAmountNotSet".to_string()
                }
            }
        }
    }
    impl From<VoteMarketError> for u32 {
        fn from(e: VoteMarketError) -> u32 {
            e as u32 + anchor_lang::error::ERROR_CODE_OFFSET
        }
    }
    impl From<VoteMarketError> for anchor_lang::error::Error {
        fn from(error_code: VoteMarketError) -> anchor_lang::error::Error {
            anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                error_name: error_code.name(),
                error_code_number: error_code.into(),
                error_msg: error_code.to_string(),
                error_origin: None,
                compared_values: None,
            })
        }
    }
    impl std::fmt::Display for VoteMarketError {
        fn fmt(
            &self,
            fmt: &mut std::fmt::Formatter<'_>,
        ) -> std::result::Result<(), std::fmt::Error> {
            match self {
                VoteMarketError::CompletedEpoch => {
                    fmt.write_fmt(format_args!("Cannot modify completed epochs"))
                }
                VoteMarketError::EpochVotingNotCompleted => {
                    fmt.write_fmt(format_args!("Epoch voting not completed"))
                }
                VoteMarketError::InvalidAllocatedVoteAmount => {
                    fmt.write_fmt(
                        format_args!(
                            "Allocated vote amount is greater than total vote amount",
                        ),
                    )
                }
                VoteMarketError::EpochOverflow => {
                    fmt.write_fmt(format_args!("Epoch overflow"))
                }
                VoteMarketError::InvalidMint => {
                    fmt.write_fmt(format_args!("Invalid vote payment mint"))
                }
                VoteMarketError::InvalidBuyer => {
                    fmt.write_fmt(
                        format_args!(
                            "The initial buyer is the only reward receiver for this epoch",
                        ),
                    )
                }
                VoteMarketError::InvalidVotePower => {
                    fmt.write_fmt(format_args!("Unable to calcualate vote power"))
                }
                VoteMarketError::MaxVoteBuyAmountNotSet => {
                    fmt.write_fmt(format_args!("Max vote buy amount not set"))
                }
            }
        }
    }
}
pub mod state {
    use anchor_lang::prelude::*;
    pub struct VoteBuy {
        pub mint: Pubkey,
        pub amount: u64,
        pub max_amount: Option<u64>,
        pub reward_receiver: Pubkey,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for VoteBuy {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "VoteBuy",
                "mint",
                &self.mint,
                "amount",
                &self.amount,
                "max_amount",
                &self.max_amount,
                "reward_receiver",
                &&self.reward_receiver,
            )
        }
    }
    impl borsh::ser::BorshSerialize for VoteBuy
    where
        Pubkey: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        Option<u64>: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.mint, writer)?;
            borsh::BorshSerialize::serialize(&self.amount, writer)?;
            borsh::BorshSerialize::serialize(&self.max_amount, writer)?;
            borsh::BorshSerialize::serialize(&self.reward_receiver, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for VoteBuy
    where
        Pubkey: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        Option<u64>: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                mint: borsh::BorshDeserialize::deserialize_reader(reader)?,
                amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                max_amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
                reward_receiver: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for VoteBuy {
        #[inline]
        fn clone(&self) -> VoteBuy {
            VoteBuy {
                mint: ::core::clone::Clone::clone(&self.mint),
                amount: ::core::clone::Clone::clone(&self.amount),
                max_amount: ::core::clone::Clone::clone(&self.max_amount),
                reward_receiver: ::core::clone::Clone::clone(&self.reward_receiver),
            }
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for VoteBuy {
        fn try_serialize<W: std::io::Write>(
            &self,
            writer: &mut W,
        ) -> anchor_lang::Result<()> {
            if writer.write_all(&[155, 17, 20, 82, 247, 189, 203, 225]).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for VoteBuy {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            if buf.len() < [155, 17, 20, 82, 247, 189, 203, 225].len() {
                return Err(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound.into(),
                );
            }
            let given_disc = &buf[..8];
            if &[155, 17, 20, 82, 247, 189, 203, 225] != given_disc {
                return Err(
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                            error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .name(),
                            error_code_number: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .into(),
                            error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .to_string(),
                            error_origin: Some(
                                anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                    filename: "programs/vote-market/src/state.rs",
                                    line: 3u32,
                                }),
                            ),
                            compared_values: None,
                        })
                        .with_account_name("VoteBuy"),
                );
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into()
                })
        }
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for VoteBuy {
        const DISCRIMINATOR: [u8; 8] = [155, 17, 20, 82, 247, 189, 203, 225];
    }
    #[automatically_derived]
    impl anchor_lang::Owner for VoteBuy {
        fn owner() -> Pubkey {
            crate::ID
        }
    }
    impl VoteBuy {
        pub const LEN: usize = 8 + 32 + 8 + 1 + 8 + 32;
    }
    pub struct VoteMarketConfig {
        pub script_authority: Pubkey,
        pub gaugemeister: Pubkey,
        pub admin: Pubkey,
        pub efficiency_ratio: u64,
    }
    impl borsh::ser::BorshSerialize for VoteMarketConfig
    where
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.script_authority, writer)?;
            borsh::BorshSerialize::serialize(&self.gaugemeister, writer)?;
            borsh::BorshSerialize::serialize(&self.admin, writer)?;
            borsh::BorshSerialize::serialize(&self.efficiency_ratio, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for VoteMarketConfig
    where
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                script_authority: borsh::BorshDeserialize::deserialize_reader(reader)?,
                gaugemeister: borsh::BorshDeserialize::deserialize_reader(reader)?,
                admin: borsh::BorshDeserialize::deserialize_reader(reader)?,
                efficiency_ratio: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for VoteMarketConfig {
        #[inline]
        fn clone(&self) -> VoteMarketConfig {
            VoteMarketConfig {
                script_authority: ::core::clone::Clone::clone(&self.script_authority),
                gaugemeister: ::core::clone::Clone::clone(&self.gaugemeister),
                admin: ::core::clone::Clone::clone(&self.admin),
                efficiency_ratio: ::core::clone::Clone::clone(&self.efficiency_ratio),
            }
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for VoteMarketConfig {
        fn try_serialize<W: std::io::Write>(
            &self,
            writer: &mut W,
        ) -> anchor_lang::Result<()> {
            if writer.write_all(&[172, 230, 34, 183, 39, 247, 10, 193]).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for VoteMarketConfig {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            if buf.len() < [172, 230, 34, 183, 39, 247, 10, 193].len() {
                return Err(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound.into(),
                );
            }
            let given_disc = &buf[..8];
            if &[172, 230, 34, 183, 39, 247, 10, 193] != given_disc {
                return Err(
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                            error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .name(),
                            error_code_number: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .into(),
                            error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .to_string(),
                            error_origin: Some(
                                anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                    filename: "programs/vote-market/src/state.rs",
                                    line: 17u32,
                                }),
                            ),
                            compared_values: None,
                        })
                        .with_account_name("VoteMarketConfig"),
                );
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into()
                })
        }
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for VoteMarketConfig {
        const DISCRIMINATOR: [u8; 8] = [172, 230, 34, 183, 39, 247, 10, 193];
    }
    #[automatically_derived]
    impl anchor_lang::Owner for VoteMarketConfig {
        fn owner() -> Pubkey {
            crate::ID
        }
    }
    impl VoteMarketConfig {
        pub const LEN: usize = 8 + 32 + 32 + 32 + 8;
    }
    pub struct AllowedMints {
        pub mints: Vec<Pubkey>,
    }
    impl borsh::ser::BorshSerialize for AllowedMints
    where
        Vec<Pubkey>: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.mints, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for AllowedMints
    where
        Vec<Pubkey>: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                mints: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for AllowedMints {
        #[inline]
        fn clone(&self) -> AllowedMints {
            AllowedMints {
                mints: ::core::clone::Clone::clone(&self.mints),
            }
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for AllowedMints {
        fn try_serialize<W: std::io::Write>(
            &self,
            writer: &mut W,
        ) -> anchor_lang::Result<()> {
            if writer.write_all(&[207, 250, 95, 68, 245, 220, 110, 31]).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for AllowedMints {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            if buf.len() < [207, 250, 95, 68, 245, 220, 110, 31].len() {
                return Err(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound.into(),
                );
            }
            let given_disc = &buf[..8];
            if &[207, 250, 95, 68, 245, 220, 110, 31] != given_disc {
                return Err(
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                            error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .name(),
                            error_code_number: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .into(),
                            error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .to_string(),
                            error_origin: Some(
                                anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                    filename: "programs/vote-market/src/state.rs",
                                    line: 29u32,
                                }),
                            ),
                            compared_values: None,
                        })
                        .with_account_name("AllowedMints"),
                );
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into()
                })
        }
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for AllowedMints {
        const DISCRIMINATOR: [u8; 8] = [207, 250, 95, 68, 245, 220, 110, 31];
    }
    #[automatically_derived]
    impl anchor_lang::Owner for AllowedMints {
        fn owner() -> Pubkey {
            crate::ID
        }
    }
    impl AllowedMints {
        pub fn len(mints: usize) -> usize {
            8 + 4 + 32 * mints
        }
    }
}
pub mod util {
    pub mod vote_math {
        use crate::errors::VoteMarketError;
        use anchor_lang::err;
        use anchor_lang::error::Error;
        pub fn get_user_payment(
            total_power: u64,
            allocated_power: u64,
            total_vote_payment: u64,
        ) -> Result<u64, Error> {
            if total_power == 0 || allocated_power == 0 || total_vote_payment == 0 {
                return Ok(0);
            }
            if allocated_power > total_power {
                return Err(
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                        error_name: VoteMarketError::InvalidAllocatedVoteAmount.name(),
                        error_code_number: VoteMarketError::InvalidAllocatedVoteAmount
                            .into(),
                        error_msg: VoteMarketError::InvalidAllocatedVoteAmount
                            .to_string(),
                        error_origin: Some(
                            anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                filename: "programs/vote-market/src/util/vote_math.rs",
                                line: 14u32,
                            }),
                        ),
                        compared_values: None,
                    }),
                );
            }
            ::u128::mul_div_u64(allocated_power, total_vote_payment, total_power)
                .ok_or(VoteMarketError::InvalidVotePower.into())
        }
    }
}
use crate::state::{AllowedMints, VoteBuy, VoteMarketConfig};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};
use gauge_state::GaugeProgram;
use locked_voter_state::LockedVoterProgram;
/// The static program ID
pub static ID: anchor_lang::solana_program::pubkey::Pubkey = anchor_lang::solana_program::pubkey::Pubkey::new_from_array([
    2u8,
    254u8,
    105u8,
    246u8,
    167u8,
    22u8,
    249u8,
    77u8,
    0u8,
    114u8,
    56u8,
    147u8,
    219u8,
    152u8,
    15u8,
    195u8,
    190u8,
    51u8,
    36u8,
    71u8,
    4u8,
    58u8,
    162u8,
    167u8,
    56u8,
    44u8,
    149u8,
    170u8,
    149u8,
    101u8,
    46u8,
    115u8,
]);
/// Confirms that a given pubkey is equivalent to the program ID
pub fn check_id(id: &anchor_lang::solana_program::pubkey::Pubkey) -> bool {
    id == &ID
}
/// Returns the program ID
pub fn id() -> anchor_lang::solana_program::pubkey::Pubkey {
    ID
}
use self::vote_market::*;
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
    let (program_id, accounts, instruction_data) = unsafe {
        ::solana_program::entrypoint::deserialize(input)
    };
    match entry(&program_id, &accounts, &instruction_data) {
        Ok(()) => ::solana_program::entrypoint::SUCCESS,
        Err(error) => error.into(),
    }
}
/// The Anchor codegen exposes a programming model where a user defines
/// a set of methods inside of a `#[program]` module in a way similar
/// to writing RPC request handlers. The macro then generates a bunch of
/// code wrapping these user defined methods into something that can be
/// executed on Solana.
///
/// These methods fall into one category for now.
///
/// Global methods - regular methods inside of the `#[program]`.
///
/// Care must be taken by the codegen to prevent collisions between
/// methods in these different namespaces. For this reason, Anchor uses
/// a variant of sighash to perform method dispatch, rather than
/// something like a simple enum variant discriminator.
///
/// The execution flow of the generated code can be roughly outlined:
///
/// * Start program via the entrypoint.
/// * Strip method identifier off the first 8 bytes of the instruction
///   data and invoke the identified method. The method identifier
///   is a variant of sighash. See docs.rs for `anchor_lang` for details.
/// * If the method identifier is an IDL identifier, execute the IDL
///   instructions, which are a special set of hardcoded instructions
///   baked into every Anchor program. Then exit.
/// * Otherwise, the method identifier is for a user defined
///   instruction, i.e., one of the methods in the user defined
///   `#[program]` module. Perform method dispatch, i.e., execute the
///   big match statement mapping method identifier to method handler
///   wrapper.
/// * Run the method handler wrapper. This wraps the code the user
///   actually wrote, deserializing the accounts, constructing the
///   context, invoking the user's code, and finally running the exit
///   routine, which typically persists account changes.
///
/// The `entry` function here, defines the standard entry to a Solana
/// program, where execution begins.
pub fn entry<'info>(
    program_id: &Pubkey,
    accounts: &'info [AccountInfo<'info>],
    data: &[u8],
) -> anchor_lang::solana_program::entrypoint::ProgramResult {
    try_entry(program_id, accounts, data)
        .map_err(|e| {
            e.log();
            e.into()
        })
}
fn try_entry<'info>(
    program_id: &Pubkey,
    accounts: &'info [AccountInfo<'info>],
    data: &[u8],
) -> anchor_lang::Result<()> {
    if *program_id != ID {
        return Err(anchor_lang::error::ErrorCode::DeclaredProgramIdMismatch.into());
    }
    if data.len() < 8 {
        return Err(anchor_lang::error::ErrorCode::InstructionMissing.into());
    }
    dispatch(program_id, accounts, data)
}
/// Module representing the program.
pub mod program {
    use super::*;
    /// Type representing the program.
    pub struct VoteMarket;
    #[automatically_derived]
    impl ::core::clone::Clone for VoteMarket {
        #[inline]
        fn clone(&self) -> VoteMarket {
            VoteMarket
        }
    }
    impl anchor_lang::Id for VoteMarket {
        fn id() -> Pubkey {
            ID
        }
    }
}
/// Performs method dispatch.
///
/// Each method in an anchor program is uniquely defined by a namespace
/// and a rust identifier (i.e., the name given to the method). These
/// two pieces can be combined to creater a method identifier,
/// specifically, Anchor uses
///
/// Sha256("<namespace>:<rust-identifier>")[..8],
///
/// where the namespace can be one type. "global" for a
/// regular instruction.
///
/// With this 8 byte identifier, Anchor performs method dispatch,
/// matching the given 8 byte identifier to the associated method
/// handler, which leads to user defined code being eventually invoked.
fn dispatch<'info>(
    program_id: &Pubkey,
    accounts: &'info [AccountInfo<'info>],
    data: &[u8],
) -> anchor_lang::Result<()> {
    let mut ix_data: &[u8] = data;
    let sighash: [u8; 8] = {
        let mut sighash: [u8; 8] = [0; 8];
        sighash.copy_from_slice(&ix_data[..8]);
        ix_data = &ix_data[8..];
        sighash
    };
    use anchor_lang::Discriminator;
    match sighash {
        instruction::CreateConfig::DISCRIMINATOR => {
            __private::__global::create_config(program_id, accounts, ix_data)
        }
        instruction::UpdateAdmin::DISCRIMINATOR => {
            __private::__global::update_admin(program_id, accounts, ix_data)
        }
        instruction::UpdateScriptAuthority::DISCRIMINATOR => {
            __private::__global::update_script_authority(program_id, accounts, ix_data)
        }
        instruction::UpdateAllowedMints::DISCRIMINATOR => {
            __private::__global::update_allowed_mints(program_id, accounts, ix_data)
        }
        instruction::IncreaseVoteBuy::DISCRIMINATOR => {
            __private::__global::increase_vote_buy(program_id, accounts, ix_data)
        }
        instruction::ClaimVotePayment::DISCRIMINATOR => {
            __private::__global::claim_vote_payment(program_id, accounts, ix_data)
        }
        instruction::ClaimRewardsAsVoteSeller::DISCRIMINATOR => {
            __private::__global::claim_rewards_as_vote_seller(
                program_id,
                accounts,
                ix_data,
            )
        }
        instruction::ClaimRewardsAsVoteBuyer::DISCRIMINATOR => {
            __private::__global::claim_rewards_as_vote_buyer(
                program_id,
                accounts,
                ix_data,
            )
        }
        instruction::Vote::DISCRIMINATOR => {
            __private::__global::vote(program_id, accounts, ix_data)
        }
        instruction::SetMaxAmount::DISCRIMINATOR => {
            __private::__global::set_max_amount(program_id, accounts, ix_data)
        }
        anchor_lang::idl::IDL_IX_TAG_LE => {
            __private::__idl::__idl_dispatch(program_id, accounts, &ix_data)
        }
        anchor_lang::event::EVENT_IX_TAG_LE => {
            Err(anchor_lang::error::ErrorCode::EventInstructionStub.into())
        }
        _ => Err(anchor_lang::error::ErrorCode::InstructionFallbackNotFound.into()),
    }
}
/// Create a private module to not clutter the program's namespace.
/// Defines an entrypoint for each individual instruction handler
/// wrapper.
mod __private {
    use super::*;
    /// __idl mod defines handlers for injected Anchor IDL instructions.
    pub mod __idl {
        use super::*;
        #[inline(never)]
        #[cfg(not(feature = "no-idl"))]
        pub fn __idl_dispatch<'info>(
            program_id: &Pubkey,
            accounts: &'info [AccountInfo<'info>],
            idl_ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            let mut accounts = accounts;
            let mut data: &[u8] = idl_ix_data;
            let ix = anchor_lang::idl::IdlInstruction::deserialize(&mut data)
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            match ix {
                anchor_lang::idl::IdlInstruction::Create { data_len } => {
                    let mut bumps = <IdlCreateAccounts as anchor_lang::Bumps>::Bumps::default();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = IdlCreateAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_create_account(program_id, &mut accounts, data_len)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::Resize { data_len } => {
                    let mut bumps = <IdlResizeAccount as anchor_lang::Bumps>::Bumps::default();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = IdlResizeAccount::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_resize_account(program_id, &mut accounts, data_len)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::Close => {
                    let mut bumps = <IdlCloseAccount as anchor_lang::Bumps>::Bumps::default();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = IdlCloseAccount::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_close_account(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::CreateBuffer => {
                    let mut bumps = <IdlCreateBuffer as anchor_lang::Bumps>::Bumps::default();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = IdlCreateBuffer::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_create_buffer(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::Write { data } => {
                    let mut bumps = <IdlAccounts as anchor_lang::Bumps>::Bumps::default();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = IdlAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_write(program_id, &mut accounts, data)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::SetAuthority { new_authority } => {
                    let mut bumps = <IdlAccounts as anchor_lang::Bumps>::Bumps::default();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = IdlAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_set_authority(program_id, &mut accounts, new_authority)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::SetBuffer => {
                    let mut bumps = <IdlSetBuffer as anchor_lang::Bumps>::Bumps::default();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = IdlSetBuffer::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_set_buffer(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
            }
            Ok(())
        }
        use anchor_lang::idl::ERASED_AUTHORITY;
        pub struct IdlAccount {
            pub authority: Pubkey,
            pub data_len: u32,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlAccount {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "IdlAccount",
                    "authority",
                    &self.authority,
                    "data_len",
                    &&self.data_len,
                )
            }
        }
        impl borsh::ser::BorshSerialize for IdlAccount
        where
            Pubkey: borsh::ser::BorshSerialize,
            u32: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.authority, writer)?;
                borsh::BorshSerialize::serialize(&self.data_len, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for IdlAccount
        where
            Pubkey: borsh::BorshDeserialize,
            u32: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    authority: borsh::BorshDeserialize::deserialize_reader(reader)?,
                    data_len: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for IdlAccount {
            #[inline]
            fn clone(&self) -> IdlAccount {
                IdlAccount {
                    authority: ::core::clone::Clone::clone(&self.authority),
                    data_len: ::core::clone::Clone::clone(&self.data_len),
                }
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountSerialize for IdlAccount {
            fn try_serialize<W: std::io::Write>(
                &self,
                writer: &mut W,
            ) -> anchor_lang::Result<()> {
                if writer.write_all(&[24, 70, 98, 191, 58, 144, 123, 158]).is_err() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDidNotSerialize.into(),
                    );
                }
                if AnchorSerialize::serialize(self, writer).is_err() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDidNotSerialize.into(),
                    );
                }
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountDeserialize for IdlAccount {
            fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                if buf.len() < [24, 70, 98, 191, 58, 144, 123, 158].len() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound
                            .into(),
                    );
                }
                let given_disc = &buf[..8];
                if &[24, 70, 98, 191, 58, 144, 123, 158] != given_disc {
                    return Err(
                        anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .name(),
                                error_code_number: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .into(),
                                error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                    .to_string(),
                                error_origin: Some(
                                    anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                        filename: "programs/vote-market/src/lib.rs",
                                        line: 14u32,
                                    }),
                                ),
                                compared_values: None,
                            })
                            .with_account_name("IdlAccount"),
                    );
                }
                Self::try_deserialize_unchecked(buf)
            }
            fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                let mut data: &[u8] = &buf[8..];
                AnchorDeserialize::deserialize(&mut data)
                    .map_err(|_| {
                        anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into()
                    })
            }
        }
        #[automatically_derived]
        impl anchor_lang::Discriminator for IdlAccount {
            const DISCRIMINATOR: [u8; 8] = [24, 70, 98, 191, 58, 144, 123, 158];
        }
        impl IdlAccount {
            pub fn address(program_id: &Pubkey) -> Pubkey {
                let program_signer = Pubkey::find_program_address(&[], program_id).0;
                Pubkey::create_with_seed(&program_signer, IdlAccount::seed(), program_id)
                    .expect("Seed is always valid")
            }
            pub fn seed() -> &'static str {
                "anchor:idl"
            }
        }
        impl anchor_lang::Owner for IdlAccount {
            fn owner() -> Pubkey {
                crate::ID
            }
        }
        pub struct IdlCreateAccounts<'info> {
            #[account(signer)]
            pub from: AccountInfo<'info>,
            #[account(mut)]
            pub to: AccountInfo<'info>,
            #[account(seeds = [], bump)]
            pub base: AccountInfo<'info>,
            pub system_program: Program<'info, System>,
            #[account(executable)]
            pub program: AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, IdlCreateAccountsBumps>
        for IdlCreateAccounts<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut IdlCreateAccountsBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let from: AccountInfo = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("from"))?;
                let to: AccountInfo = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("to"))?;
                let base: AccountInfo = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("base"))?;
                let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("system_program"))?;
                let program: AccountInfo = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("program"))?;
                if !&from.is_signer {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSigner,
                            )
                            .with_account_name("from"),
                    );
                }
                if !&to.is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("to"),
                    );
                }
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[],
                    &__program_id,
                );
                __bumps.base = __bump;
                if base.key() != __pda_address {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSeeds,
                            )
                            .with_account_name("base")
                            .with_pubkeys((base.key(), __pda_address)),
                    );
                }
                if !&program.executable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintExecutable,
                            )
                            .with_account_name("program"),
                    );
                }
                Ok(IdlCreateAccounts {
                    from,
                    to,
                    base,
                    system_program,
                    program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for IdlCreateAccounts<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.from.to_account_infos());
                account_infos.extend(self.to.to_account_infos());
                account_infos.extend(self.base.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos.extend(self.program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlCreateAccounts<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.from.to_account_metas(Some(true)));
                account_metas.extend(self.to.to_account_metas(None));
                account_metas.extend(self.base.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas.extend(self.program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for IdlCreateAccounts<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.to, program_id)
                    .map_err(|e| e.with_account_name("to"))?;
                Ok(())
            }
        }
        pub struct IdlCreateAccountsBumps {
            pub base: u8,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlCreateAccountsBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "IdlCreateAccountsBumps",
                    "base",
                    &&self.base,
                )
            }
        }
        impl Default for IdlCreateAccountsBumps {
            fn default() -> Self {
                IdlCreateAccountsBumps {
                    base: u8::MAX,
                }
            }
        }
        impl<'info> anchor_lang::Bumps for IdlCreateAccounts<'info>
        where
            'info: 'info,
        {
            type Bumps = IdlCreateAccountsBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_idl_create_accounts {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`IdlCreateAccounts`].
            pub struct IdlCreateAccounts {
                pub from: Pubkey,
                pub to: Pubkey,
                pub base: Pubkey,
                pub system_program: Pubkey,
                pub program: Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlCreateAccounts
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.from, writer)?;
                    borsh::BorshSerialize::serialize(&self.to, writer)?;
                    borsh::BorshSerialize::serialize(&self.base, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    borsh::BorshSerialize::serialize(&self.program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlCreateAccounts {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.from,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.to,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.base,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.system_program,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.program,
                                false,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_idl_create_accounts {
            use super::*;
            /// Generated CPI struct of the accounts for [`IdlCreateAccounts`].
            pub struct IdlCreateAccounts<'info> {
                pub from: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub to: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub base: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlCreateAccounts<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.from),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.to),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.base),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.system_program),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.program),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for IdlCreateAccounts<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.from),
                        );
                    account_infos
                        .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.to));
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.base),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.system_program,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.program),
                        );
                    account_infos
                }
            }
        }
        pub struct IdlAccounts<'info> {
            #[account(mut, has_one = authority)]
            pub idl: Account<'info, IdlAccount>,
            #[account(constraint = authority.key!= &ERASED_AUTHORITY)]
            pub authority: Signer<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, IdlAccountsBumps> for IdlAccounts<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut IdlAccountsBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let idl: anchor_lang::accounts::account::Account<IdlAccount> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("idl"))?;
                let authority: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("authority"))?;
                if !AsRef::<AccountInfo>::as_ref(&idl).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("idl"),
                    );
                }
                {
                    let my_key = idl.authority;
                    let target_key = authority.key();
                    if my_key != target_key {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintHasOne,
                                )
                                .with_account_name("idl")
                                .with_pubkeys((my_key, target_key)),
                        );
                    }
                }
                if !(authority.key != &ERASED_AUTHORITY) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRaw,
                            )
                            .with_account_name("authority"),
                    );
                }
                Ok(IdlAccounts { idl, authority })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for IdlAccounts<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.idl.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlAccounts<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.idl.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for IdlAccounts<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.idl, program_id)
                    .map_err(|e| e.with_account_name("idl"))?;
                Ok(())
            }
        }
        pub struct IdlAccountsBumps {}
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlAccountsBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "IdlAccountsBumps")
            }
        }
        impl Default for IdlAccountsBumps {
            fn default() -> Self {
                IdlAccountsBumps {}
            }
        }
        impl<'info> anchor_lang::Bumps for IdlAccounts<'info>
        where
            'info: 'info,
        {
            type Bumps = IdlAccountsBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_idl_accounts {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`IdlAccounts`].
            pub struct IdlAccounts {
                pub idl: Pubkey,
                pub authority: Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlAccounts
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.idl, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlAccounts {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.idl,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.authority,
                                true,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_idl_accounts {
            use super::*;
            /// Generated CPI struct of the accounts for [`IdlAccounts`].
            pub struct IdlAccounts<'info> {
                pub idl: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlAccounts<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.idl),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.authority),
                                true,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for IdlAccounts<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.idl),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.authority,
                            ),
                        );
                    account_infos
                }
            }
        }
        pub struct IdlResizeAccount<'info> {
            #[account(mut, has_one = authority)]
            pub idl: Account<'info, IdlAccount>,
            #[account(mut, constraint = authority.key!= &ERASED_AUTHORITY)]
            pub authority: Signer<'info>,
            pub system_program: Program<'info, System>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, IdlResizeAccountBumps>
        for IdlResizeAccount<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut IdlResizeAccountBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let idl: anchor_lang::accounts::account::Account<IdlAccount> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("idl"))?;
                let authority: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("authority"))?;
                let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("system_program"))?;
                if !AsRef::<AccountInfo>::as_ref(&idl).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("idl"),
                    );
                }
                {
                    let my_key = idl.authority;
                    let target_key = authority.key();
                    if my_key != target_key {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintHasOne,
                                )
                                .with_account_name("idl")
                                .with_pubkeys((my_key, target_key)),
                        );
                    }
                }
                if !AsRef::<AccountInfo>::as_ref(&authority).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("authority"),
                    );
                }
                if !(authority.key != &ERASED_AUTHORITY) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRaw,
                            )
                            .with_account_name("authority"),
                    );
                }
                Ok(IdlResizeAccount {
                    idl,
                    authority,
                    system_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for IdlResizeAccount<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.idl.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlResizeAccount<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.idl.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for IdlResizeAccount<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.idl, program_id)
                    .map_err(|e| e.with_account_name("idl"))?;
                anchor_lang::AccountsExit::exit(&self.authority, program_id)
                    .map_err(|e| e.with_account_name("authority"))?;
                Ok(())
            }
        }
        pub struct IdlResizeAccountBumps {}
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlResizeAccountBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "IdlResizeAccountBumps")
            }
        }
        impl Default for IdlResizeAccountBumps {
            fn default() -> Self {
                IdlResizeAccountBumps {}
            }
        }
        impl<'info> anchor_lang::Bumps for IdlResizeAccount<'info>
        where
            'info: 'info,
        {
            type Bumps = IdlResizeAccountBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_idl_resize_account {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`IdlResizeAccount`].
            pub struct IdlResizeAccount {
                pub idl: Pubkey,
                pub authority: Pubkey,
                pub system_program: Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlResizeAccount
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.idl, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlResizeAccount {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.idl,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.authority,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.system_program,
                                false,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_idl_resize_account {
            use super::*;
            /// Generated CPI struct of the accounts for [`IdlResizeAccount`].
            pub struct IdlResizeAccount<'info> {
                pub idl: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlResizeAccount<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.idl),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.authority),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.system_program),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for IdlResizeAccount<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.idl),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.authority,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.system_program,
                            ),
                        );
                    account_infos
                }
            }
        }
        pub struct IdlCreateBuffer<'info> {
            #[account(zero)]
            pub buffer: Account<'info, IdlAccount>,
            #[account(constraint = authority.key!= &ERASED_AUTHORITY)]
            pub authority: Signer<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, IdlCreateBufferBumps>
        for IdlCreateBuffer<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut IdlCreateBufferBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                if __accounts.is_empty() {
                    return Err(
                        anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into(),
                    );
                }
                let buffer = &__accounts[0];
                *__accounts = &__accounts[1..];
                let authority: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("authority"))?;
                let __anchor_rent = Rent::get()?;
                let buffer: anchor_lang::accounts::account::Account<IdlAccount> = {
                    let mut __data: &[u8] = &buffer.try_borrow_data()?;
                    let mut __disc_bytes = [0u8; 8];
                    __disc_bytes.copy_from_slice(&__data[..8]);
                    let __discriminator = u64::from_le_bytes(__disc_bytes);
                    if __discriminator != 0 {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintZero,
                                )
                                .with_account_name("buffer"),
                        );
                    }
                    match anchor_lang::accounts::account::Account::try_from_unchecked(
                        &buffer,
                    ) {
                        Ok(val) => val,
                        Err(e) => return Err(e.with_account_name("buffer")),
                    }
                };
                if !AsRef::<AccountInfo>::as_ref(&buffer).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("buffer"),
                    );
                }
                if !__anchor_rent
                    .is_exempt(
                        buffer.to_account_info().lamports(),
                        buffer.to_account_info().try_data_len()?,
                    )
                {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRentExempt,
                            )
                            .with_account_name("buffer"),
                    );
                }
                if !(authority.key != &ERASED_AUTHORITY) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRaw,
                            )
                            .with_account_name("authority"),
                    );
                }
                Ok(IdlCreateBuffer {
                    buffer,
                    authority,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for IdlCreateBuffer<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.buffer.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlCreateBuffer<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.buffer.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for IdlCreateBuffer<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.buffer, program_id)
                    .map_err(|e| e.with_account_name("buffer"))?;
                Ok(())
            }
        }
        pub struct IdlCreateBufferBumps {}
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlCreateBufferBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "IdlCreateBufferBumps")
            }
        }
        impl Default for IdlCreateBufferBumps {
            fn default() -> Self {
                IdlCreateBufferBumps {}
            }
        }
        impl<'info> anchor_lang::Bumps for IdlCreateBuffer<'info>
        where
            'info: 'info,
        {
            type Bumps = IdlCreateBufferBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_idl_create_buffer {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`IdlCreateBuffer`].
            pub struct IdlCreateBuffer {
                pub buffer: Pubkey,
                pub authority: Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlCreateBuffer
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.buffer, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlCreateBuffer {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.buffer,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.authority,
                                true,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_idl_create_buffer {
            use super::*;
            /// Generated CPI struct of the accounts for [`IdlCreateBuffer`].
            pub struct IdlCreateBuffer<'info> {
                pub buffer: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlCreateBuffer<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.buffer),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.authority),
                                true,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for IdlCreateBuffer<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.buffer),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.authority,
                            ),
                        );
                    account_infos
                }
            }
        }
        pub struct IdlSetBuffer<'info> {
            #[account(mut, constraint = buffer.authority = = idl.authority)]
            pub buffer: Account<'info, IdlAccount>,
            #[account(mut, has_one = authority)]
            pub idl: Account<'info, IdlAccount>,
            #[account(constraint = authority.key!= &ERASED_AUTHORITY)]
            pub authority: Signer<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, IdlSetBufferBumps>
        for IdlSetBuffer<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut IdlSetBufferBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let buffer: anchor_lang::accounts::account::Account<IdlAccount> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("buffer"))?;
                let idl: anchor_lang::accounts::account::Account<IdlAccount> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("idl"))?;
                let authority: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("authority"))?;
                if !AsRef::<AccountInfo>::as_ref(&buffer).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("buffer"),
                    );
                }
                if !(buffer.authority == idl.authority) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRaw,
                            )
                            .with_account_name("buffer"),
                    );
                }
                if !AsRef::<AccountInfo>::as_ref(&idl).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("idl"),
                    );
                }
                {
                    let my_key = idl.authority;
                    let target_key = authority.key();
                    if my_key != target_key {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintHasOne,
                                )
                                .with_account_name("idl")
                                .with_pubkeys((my_key, target_key)),
                        );
                    }
                }
                if !(authority.key != &ERASED_AUTHORITY) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRaw,
                            )
                            .with_account_name("authority"),
                    );
                }
                Ok(IdlSetBuffer {
                    buffer,
                    idl,
                    authority,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for IdlSetBuffer<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.buffer.to_account_infos());
                account_infos.extend(self.idl.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlSetBuffer<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.buffer.to_account_metas(None));
                account_metas.extend(self.idl.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for IdlSetBuffer<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.buffer, program_id)
                    .map_err(|e| e.with_account_name("buffer"))?;
                anchor_lang::AccountsExit::exit(&self.idl, program_id)
                    .map_err(|e| e.with_account_name("idl"))?;
                Ok(())
            }
        }
        pub struct IdlSetBufferBumps {}
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlSetBufferBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "IdlSetBufferBumps")
            }
        }
        impl Default for IdlSetBufferBumps {
            fn default() -> Self {
                IdlSetBufferBumps {}
            }
        }
        impl<'info> anchor_lang::Bumps for IdlSetBuffer<'info>
        where
            'info: 'info,
        {
            type Bumps = IdlSetBufferBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_idl_set_buffer {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`IdlSetBuffer`].
            pub struct IdlSetBuffer {
                pub buffer: Pubkey,
                pub idl: Pubkey,
                pub authority: Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlSetBuffer
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.buffer, writer)?;
                    borsh::BorshSerialize::serialize(&self.idl, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlSetBuffer {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.buffer,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.idl,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.authority,
                                true,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_idl_set_buffer {
            use super::*;
            /// Generated CPI struct of the accounts for [`IdlSetBuffer`].
            pub struct IdlSetBuffer<'info> {
                pub buffer: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub idl: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlSetBuffer<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.buffer),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.idl),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.authority),
                                true,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for IdlSetBuffer<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.buffer),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.idl),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.authority,
                            ),
                        );
                    account_infos
                }
            }
        }
        pub struct IdlCloseAccount<'info> {
            #[account(mut, has_one = authority, close = sol_destination)]
            pub account: Account<'info, IdlAccount>,
            #[account(constraint = authority.key!= &ERASED_AUTHORITY)]
            pub authority: Signer<'info>,
            #[account(mut)]
            pub sol_destination: AccountInfo<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info, IdlCloseAccountBumps>
        for IdlCloseAccount<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >],
                __ix_data: &[u8],
                __bumps: &mut IdlCloseAccountBumps,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let account: anchor_lang::accounts::account::Account<IdlAccount> = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("account"))?;
                let authority: Signer = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("authority"))?;
                let sol_destination: AccountInfo = anchor_lang::Accounts::try_accounts(
                        __program_id,
                        __accounts,
                        __ix_data,
                        __bumps,
                        __reallocs,
                    )
                    .map_err(|e| e.with_account_name("sol_destination"))?;
                if !AsRef::<AccountInfo>::as_ref(&account).is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("account"),
                    );
                }
                {
                    let my_key = account.authority;
                    let target_key = authority.key();
                    if my_key != target_key {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintHasOne,
                                )
                                .with_account_name("account")
                                .with_pubkeys((my_key, target_key)),
                        );
                    }
                }
                {
                    if account.key() == sol_destination.key() {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintClose,
                                )
                                .with_account_name("account"),
                        );
                    }
                }
                if !(authority.key != &ERASED_AUTHORITY) {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintRaw,
                            )
                            .with_account_name("authority"),
                    );
                }
                if !&sol_destination.is_writable {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMut,
                            )
                            .with_account_name("sol_destination"),
                    );
                }
                Ok(IdlCloseAccount {
                    account,
                    authority,
                    sol_destination,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for IdlCloseAccount<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.account.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos.extend(self.sol_destination.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for IdlCloseAccount<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.account.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas.extend(self.sol_destination.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for IdlCloseAccount<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                {
                    let sol_destination = &self.sol_destination;
                    anchor_lang::AccountsClose::close(
                            &self.account,
                            sol_destination.to_account_info(),
                        )
                        .map_err(|e| e.with_account_name("account"))?;
                }
                anchor_lang::AccountsExit::exit(&self.sol_destination, program_id)
                    .map_err(|e| e.with_account_name("sol_destination"))?;
                Ok(())
            }
        }
        pub struct IdlCloseAccountBumps {}
        #[automatically_derived]
        impl ::core::fmt::Debug for IdlCloseAccountBumps {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "IdlCloseAccountBumps")
            }
        }
        impl Default for IdlCloseAccountBumps {
            fn default() -> Self {
                IdlCloseAccountBumps {}
            }
        }
        impl<'info> anchor_lang::Bumps for IdlCloseAccount<'info>
        where
            'info: 'info,
        {
            type Bumps = IdlCloseAccountBumps;
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_idl_close_account {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`IdlCloseAccount`].
            pub struct IdlCloseAccount {
                pub account: Pubkey,
                pub authority: Pubkey,
                pub sol_destination: Pubkey,
            }
            impl borsh::ser::BorshSerialize for IdlCloseAccount
            where
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
                Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.account, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    borsh::BorshSerialize::serialize(&self.sol_destination, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for IdlCloseAccount {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.account,
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                self.authority,
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                self.sol_destination,
                                false,
                            ),
                        );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_idl_close_account {
            use super::*;
            /// Generated CPI struct of the accounts for [`IdlCloseAccount`].
            pub struct IdlCloseAccount<'info> {
                pub account: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
                pub sol_destination: anchor_lang::solana_program::account_info::AccountInfo<
                    'info,
                >,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for IdlCloseAccount<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.account),
                                false,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                                anchor_lang::Key::key(&self.authority),
                                true,
                            ),
                        );
                    account_metas
                        .push(
                            anchor_lang::solana_program::instruction::AccountMeta::new(
                                anchor_lang::Key::key(&self.sol_destination),
                                false,
                            ),
                        );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for IdlCloseAccount<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(&self.account),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.authority,
                            ),
                        );
                    account_infos
                        .extend(
                            anchor_lang::ToAccountInfos::to_account_infos(
                                &self.sol_destination,
                            ),
                        );
                    account_infos
                }
            }
        }
        use std::cell::{Ref, RefMut};
        pub trait IdlTrailingData<'info> {
            fn trailing_data(self) -> Ref<'info, [u8]>;
            fn trailing_data_mut(self) -> RefMut<'info, [u8]>;
        }
        impl<'a, 'info: 'a> IdlTrailingData<'a> for &'a Account<'info, IdlAccount> {
            fn trailing_data(self) -> Ref<'a, [u8]> {
                let info: &AccountInfo<'info> = self.as_ref();
                Ref::map(info.try_borrow_data().unwrap(), |d| &d[44..])
            }
            fn trailing_data_mut(self) -> RefMut<'a, [u8]> {
                let info: &AccountInfo<'info> = self.as_ref();
                RefMut::map(info.try_borrow_mut_data().unwrap(), |d| &mut d[44..])
            }
        }
        #[inline(never)]
        pub fn __idl_create_account(
            program_id: &Pubkey,
            accounts: &mut IdlCreateAccounts,
            data_len: u64,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlCreateAccount");
            if program_id != accounts.program.key {
                return Err(
                    anchor_lang::error::ErrorCode::IdlInstructionInvalidProgram.into(),
                );
            }
            let from = accounts.from.key;
            let (base, nonce) = Pubkey::find_program_address(&[], program_id);
            let seed = IdlAccount::seed();
            let owner = accounts.program.key;
            let to = Pubkey::create_with_seed(&base, seed, owner).unwrap();
            let space = std::cmp::min(8 + 32 + 4 + data_len as usize, 10_000);
            let rent = Rent::get()?;
            let lamports = rent.minimum_balance(space);
            let seeds = &[&[nonce][..]];
            let ix = anchor_lang::solana_program::system_instruction::create_account_with_seed(
                from,
                &to,
                &base,
                seed,
                lamports,
                space as u64,
                owner,
            );
            anchor_lang::solana_program::program::invoke_signed(
                &ix,
                &[
                    accounts.from.clone(),
                    accounts.to.clone(),
                    accounts.base.clone(),
                    accounts.system_program.to_account_info(),
                ],
                &[seeds],
            )?;
            let mut idl_account = {
                let mut account_data = accounts.to.try_borrow_data()?;
                let mut account_data_slice: &[u8] = &account_data;
                IdlAccount::try_deserialize_unchecked(&mut account_data_slice)?
            };
            idl_account.authority = *accounts.from.key;
            let mut data = accounts.to.try_borrow_mut_data()?;
            let dst: &mut [u8] = &mut data;
            let mut cursor = std::io::Cursor::new(dst);
            idl_account.try_serialize(&mut cursor)?;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_resize_account(
            program_id: &Pubkey,
            accounts: &mut IdlResizeAccount,
            data_len: u64,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlResizeAccount");
            let data_len: usize = data_len as usize;
            if accounts.idl.data_len != 0 {
                return Err(anchor_lang::error::ErrorCode::IdlAccountNotEmpty.into());
            }
            let idl_ref = AsRef::<AccountInfo>::as_ref(&accounts.idl);
            let new_account_space = idl_ref
                .data_len()
                .checked_add(
                    std::cmp::min(
                        data_len
                            .checked_sub(idl_ref.data_len())
                            .expect(
                                "data_len should always be >= the current account space",
                            ),
                        10_000,
                    ),
                )
                .unwrap();
            if new_account_space > idl_ref.data_len() {
                let sysvar_rent = Rent::get()?;
                let new_rent_minimum = sysvar_rent.minimum_balance(new_account_space);
                anchor_lang::system_program::transfer(
                    anchor_lang::context::CpiContext::new(
                        accounts.system_program.to_account_info(),
                        anchor_lang::system_program::Transfer {
                            from: accounts.authority.to_account_info(),
                            to: accounts.idl.to_account_info().clone(),
                        },
                    ),
                    new_rent_minimum.checked_sub(idl_ref.lamports()).unwrap(),
                )?;
                idl_ref.realloc(new_account_space, false)?;
            }
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_close_account(
            program_id: &Pubkey,
            accounts: &mut IdlCloseAccount,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlCloseAccount");
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_create_buffer(
            program_id: &Pubkey,
            accounts: &mut IdlCreateBuffer,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlCreateBuffer");
            let mut buffer = &mut accounts.buffer;
            buffer.authority = *accounts.authority.key;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_write(
            program_id: &Pubkey,
            accounts: &mut IdlAccounts,
            idl_data: Vec<u8>,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlWrite");
            let prev_len: usize = ::std::convert::TryInto::<
                usize,
            >::try_into(accounts.idl.data_len)
                .unwrap();
            let new_len: usize = prev_len.checked_add(idl_data.len()).unwrap() as usize;
            accounts
                .idl
                .data_len = accounts
                .idl
                .data_len
                .checked_add(
                    ::std::convert::TryInto::<u32>::try_into(idl_data.len()).unwrap(),
                )
                .unwrap();
            use IdlTrailingData;
            let mut idl_bytes = accounts.idl.trailing_data_mut();
            let idl_expansion = &mut idl_bytes[prev_len..new_len];
            if idl_expansion.len() != idl_data.len() {
                return Err(
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                            error_name: anchor_lang::error::ErrorCode::RequireEqViolated
                                .name(),
                            error_code_number: anchor_lang::error::ErrorCode::RequireEqViolated
                                .into(),
                            error_msg: anchor_lang::error::ErrorCode::RequireEqViolated
                                .to_string(),
                            error_origin: Some(
                                anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                    filename: "programs/vote-market/src/lib.rs",
                                    line: 14u32,
                                }),
                            ),
                            compared_values: None,
                        })
                        .with_values((idl_expansion.len(), idl_data.len())),
                );
            }
            idl_expansion.copy_from_slice(&idl_data[..]);
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_set_authority(
            program_id: &Pubkey,
            accounts: &mut IdlAccounts,
            new_authority: Pubkey,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlSetAuthority");
            accounts.idl.authority = new_authority;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_set_buffer(
            program_id: &Pubkey,
            accounts: &mut IdlSetBuffer,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlSetBuffer");
            accounts.idl.data_len = accounts.buffer.data_len;
            use IdlTrailingData;
            let buffer_len = ::std::convert::TryInto::<
                usize,
            >::try_into(accounts.buffer.data_len)
                .unwrap();
            let mut target = accounts.idl.trailing_data_mut();
            let source = &accounts.buffer.trailing_data()[..buffer_len];
            if target.len() < buffer_len {
                return Err(
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                            error_name: anchor_lang::error::ErrorCode::RequireGteViolated
                                .name(),
                            error_code_number: anchor_lang::error::ErrorCode::RequireGteViolated
                                .into(),
                            error_msg: anchor_lang::error::ErrorCode::RequireGteViolated
                                .to_string(),
                            error_origin: Some(
                                anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                    filename: "programs/vote-market/src/lib.rs",
                                    line: 14u32,
                                }),
                            ),
                            compared_values: None,
                        })
                        .with_values((target.len(), buffer_len)),
                );
            }
            target[..buffer_len].copy_from_slice(source);
            Ok(())
        }
    }
    /// __global mod defines wrapped handlers for global instructions.
    pub mod __global {
        use super::*;
        #[inline(never)]
        pub fn create_config<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: CreateConfig");
            let ix = instruction::CreateConfig::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::CreateConfig {
                mints,
                efficiency_ratio,
                script_authority,
            } = ix;
            let mut __bumps = <CreateConfig as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = CreateConfig::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = vote_market::create_config(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
                mints,
                efficiency_ratio,
                script_authority,
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn update_admin<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: UpdateAdmin");
            let ix = instruction::UpdateAdmin::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::UpdateAdmin { admin } = ix;
            let mut __bumps = <UpdateAdmin as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = UpdateAdmin::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = vote_market::update_admin(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
                admin,
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn update_script_authority<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: UpdateScriptAuthority");
            let ix = instruction::UpdateScriptAuthority::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::UpdateScriptAuthority { script_authority } = ix;
            let mut __bumps = <UpdateScriptAuthority as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = UpdateScriptAuthority::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = vote_market::update_script_authority(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
                script_authority,
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn update_allowed_mints<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: UpdateAllowedMints");
            let ix = instruction::UpdateAllowedMints::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::UpdateAllowedMints { allowed_mints } = ix;
            let mut __bumps = <UpdateAllowedMints as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = UpdateAllowedMints::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = vote_market::update_allowed_mints(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
                allowed_mints,
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn increase_vote_buy<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IncreaseVoteBuy");
            let ix = instruction::IncreaseVoteBuy::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::IncreaseVoteBuy { epoch, amount } = ix;
            let mut __bumps = <IncreaseVoteBuy as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = IncreaseVoteBuy::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = vote_market::increase_vote_buy(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
                epoch,
                amount,
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn claim_vote_payment<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: ClaimVotePayment");
            let ix = instruction::ClaimVotePayment::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::ClaimVotePayment { epoch } = ix;
            let mut __bumps = <ClaimVotePayment as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = ClaimVotePayment::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = vote_market::claim_vote_payment(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
                epoch,
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn claim_rewards_as_vote_seller<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: ClaimRewardsAsVoteSeller");
            let ix = instruction::ClaimRewardsAsVoteSeller::deserialize(
                    &mut &__ix_data[..],
                )
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::ClaimRewardsAsVoteSeller = ix;
            let mut __bumps = <Initialize as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = Initialize::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = vote_market::claim_rewards_as_vote_seller(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn claim_rewards_as_vote_buyer<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: ClaimRewardsAsVoteBuyer");
            let ix = instruction::ClaimRewardsAsVoteBuyer::deserialize(
                    &mut &__ix_data[..],
                )
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::ClaimRewardsAsVoteBuyer = ix;
            let mut __bumps = <Initialize as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = Initialize::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = vote_market::claim_rewards_as_vote_buyer(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn vote<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: Vote");
            let ix = instruction::Vote::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::Vote { weight } = ix;
            let mut __bumps = <Vote as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = Vote::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = vote_market::vote(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
                weight,
            )?;
            __accounts.exit(__program_id)
        }
        #[inline(never)]
        pub fn set_max_amount<'info>(
            __program_id: &Pubkey,
            __accounts: &'info [AccountInfo<'info>],
            __ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: SetMaxAmount");
            let ix = instruction::SetMaxAmount::deserialize(&mut &__ix_data[..])
                .map_err(|_| {
                    anchor_lang::error::ErrorCode::InstructionDidNotDeserialize
                })?;
            let instruction::SetMaxAmount { epoch, max_amount } = ix;
            let mut __bumps = <SetMaxAmount as anchor_lang::Bumps>::Bumps::default();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut __remaining_accounts: &[AccountInfo] = __accounts;
            let mut __accounts = SetMaxAmount::try_accounts(
                __program_id,
                &mut __remaining_accounts,
                __ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = vote_market::set_max_amount(
                anchor_lang::context::Context::new(
                    __program_id,
                    &mut __accounts,
                    __remaining_accounts,
                    __bumps,
                ),
                epoch,
                max_amount,
            )?;
            __accounts.exit(__program_id)
        }
    }
}
pub mod vote_market {
    use super::*;
    use crate::util::vote_math::get_user_payment;
    use anchor_lang::solana_program;
    use anchor_lang::solana_program::program::{invoke, invoke_signed};
    use anchor_lang::solana_program::system_instruction;
    use anchor_spl::token::spl_token;
    use std::cmp::min;
    pub fn create_config(
        ctx: Context<CreateConfig>,
        mints: Vec<Pubkey>,
        efficiency_ratio: u64,
        script_authority: Pubkey,
    ) -> Result<()> {
        ctx.accounts.config.script_authority = script_authority;
        ctx.accounts.config.gaugemeister = ctx.accounts.gaugemeister.key();
        ctx.accounts.allowed_mints.mints = mints;
        ctx.accounts.config.admin = ctx.accounts.payer.key();
        ctx.accounts.config.efficiency_ratio = efficiency_ratio;
        Ok(())
    }
    pub fn update_admin(ctx: Context<UpdateAdmin>, admin: Pubkey) -> Result<()> {
        ctx.accounts.config.admin = admin;
        Ok(())
    }
    pub fn update_script_authority(
        ctx: Context<UpdateScriptAuthority>,
        script_authority: Pubkey,
    ) -> Result<()> {
        ctx.accounts.config.script_authority = script_authority;
        Ok(())
    }
    pub fn update_allowed_mints(
        ctx: Context<UpdateAllowedMints>,
        allowed_mints: Vec<Pubkey>,
    ) -> Result<()> {
        let allowed_mints_size = AllowedMints::len(
            ctx.accounts.allowed_mints.mints.len(),
        );
        let next_allowed_mints_size = AllowedMints::len(allowed_mints.len());
        if next_allowed_mints_size > allowed_mints_size {
            let allowed_mints_account_info = ctx
                .accounts
                .allowed_mints
                .to_account_info();
            allowed_mints_account_info.realloc(next_allowed_mints_size, false)?;
            let rent = Rent::get()?;
            let next_rent_exemption = rent.minimum_balance(next_allowed_mints_size);
            if allowed_mints_account_info.lamports() < next_rent_exemption {
                let required_lamports = next_rent_exemption
                    - allowed_mints_account_info.lamports();
                let transfer_rent = system_instruction::transfer(
                    ctx.accounts.admin.key,
                    &ctx.accounts.allowed_mints.key(),
                    required_lamports,
                );
                invoke(
                    &transfer_rent,
                    &[
                        ctx.accounts.admin.to_account_info(),
                        ctx.accounts.allowed_mints.to_account_info(),
                        ctx.accounts.system_program.to_account_info(),
                    ],
                )?;
            }
        }
        ctx.accounts.allowed_mints.mints = allowed_mints;
        Ok(())
    }
    pub fn increase_vote_buy(
        ctx: Context<IncreaseVoteBuy>,
        epoch: u32,
        amount: u64,
    ) -> Result<()> {
        if ctx.accounts.buyer.key() == Pubkey::default() {
            return Err(
                anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                    error_name: errors::VoteMarketError::InvalidBuyer.name(),
                    error_code_number: errors::VoteMarketError::InvalidBuyer.into(),
                    error_msg: errors::VoteMarketError::InvalidBuyer.to_string(),
                    error_origin: Some(
                        anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                            filename: "programs/vote-market/src/lib.rs",
                            line: 87u32,
                        }),
                    ),
                    compared_values: None,
                }),
            );
        }
        if ctx.accounts.mint.key() == Pubkey::default() {
            return Err(
                anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                    error_name: errors::VoteMarketError::InvalidMint.name(),
                    error_code_number: errors::VoteMarketError::InvalidMint.into(),
                    error_msg: errors::VoteMarketError::InvalidMint.to_string(),
                    error_origin: Some(
                        anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                            filename: "programs/vote-market/src/lib.rs",
                            line: 90u32,
                        }),
                    ),
                    compared_values: None,
                }),
            );
        }
        if ctx.accounts.vote_buy.reward_receiver == Pubkey::default()
            && ctx.accounts.vote_buy.mint == Pubkey::default()
        {
            ctx.accounts.vote_buy.reward_receiver = ctx.accounts.buyer.key();
            ctx.accounts.vote_buy.mint = ctx.accounts.mint.key();
        }
        if ctx.accounts.vote_buy.reward_receiver != ctx.accounts.buyer.key() {
            return Err(
                anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                    error_name: errors::VoteMarketError::InvalidBuyer.name(),
                    error_code_number: errors::VoteMarketError::InvalidBuyer.into(),
                    error_msg: errors::VoteMarketError::InvalidBuyer.to_string(),
                    error_origin: Some(
                        anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                            filename: "programs/vote-market/src/lib.rs",
                            line: 99u32,
                        }),
                    ),
                    compared_values: None,
                }),
            );
        }
        if ctx.accounts.vote_buy.mint != ctx.accounts.mint.key() {
            return Err(
                anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                    error_name: errors::VoteMarketError::InvalidMint.name(),
                    error_code_number: errors::VoteMarketError::InvalidMint.into(),
                    error_msg: errors::VoteMarketError::InvalidMint.to_string(),
                    error_origin: Some(
                        anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                            filename: "programs/vote-market/src/lib.rs",
                            line: 102u32,
                        }),
                    ),
                    compared_values: None,
                }),
            );
        }
        if ctx.accounts.gaugemeister.current_rewards_epoch + 1 > epoch {
            return Err(
                anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                    error_name: errors::VoteMarketError::CompletedEpoch.name(),
                    error_code_number: errors::VoteMarketError::CompletedEpoch.into(),
                    error_msg: errors::VoteMarketError::CompletedEpoch.to_string(),
                    error_origin: Some(
                        anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                            filename: "programs/vote-market/src/lib.rs",
                            line: 106u32,
                        }),
                    ),
                    compared_values: None,
                }),
            );
        }
        ctx.accounts
            .allowed_mints
            .mints
            .iter()
            .find(|mint| mint == &&ctx.accounts.mint.key())
            .ok_or::<Error>(errors::VoteMarketError::InvalidMint.into())?;
        let transfer_ix = spl_token::instruction::transfer(
            &ctx.accounts.token_program.key(),
            &ctx.accounts.buyer_token_account.key(),
            &ctx.accounts.token_vault.key(),
            &ctx.accounts.buyer.key(),
            &[],
            amount,
        )?;
        invoke(
            &transfer_ix,
            &[
                ctx.accounts.buyer_token_account.to_account_info(),
                ctx.accounts.token_vault.to_account_info(),
                ctx.accounts.buyer.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
            ],
        )?;
        ctx.accounts.vote_buy.amount += amount;
        Ok(())
    }
    pub fn claim_vote_payment(ctx: Context<ClaimVotePayment>, epoch: u32) -> Result<()> {
        if epoch > ctx.accounts.gaugemeister.current_rewards_epoch {
            return Err(
                anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                    error_name: errors::VoteMarketError::EpochVotingNotCompleted.name(),
                    error_code_number: errors::VoteMarketError::EpochVotingNotCompleted
                        .into(),
                    error_msg: errors::VoteMarketError::EpochVotingNotCompleted
                        .to_string(),
                    error_origin: Some(
                        anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                            filename: "programs/vote-market/src/lib.rs",
                            line: 138u32,
                        }),
                    ),
                    compared_values: None,
                }),
            );
        }
        let total_power = ctx.accounts.epoch_gauge.total_power;
        let allocated_power = ctx.accounts.epoch_gauge_vote.allocated_power;
        let vote_buy = &ctx.accounts.vote_buy;
        let total_vote_payment = match vote_buy.max_amount {
            Some(max_amount) => min(max_amount, vote_buy.amount),
            None => {
                return Err(
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                        error_name: errors::VoteMarketError::MaxVoteBuyAmountNotSet
                            .name(),
                        error_code_number: errors::VoteMarketError::MaxVoteBuyAmountNotSet
                            .into(),
                        error_msg: errors::VoteMarketError::MaxVoteBuyAmountNotSet
                            .to_string(),
                        error_origin: Some(
                            anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                filename: "programs/vote-market/src/lib.rs",
                                line: 149u32,
                            }),
                        ),
                        compared_values: None,
                    }),
                );
            }
        };
        let payment_to_user = get_user_payment(
            total_power,
            total_vote_payment,
            allocated_power,
        )?;
        let transfer_ix = spl_token::instruction::transfer(
            &ctx.accounts.token_program.key(),
            &ctx.accounts.token_vault.key(),
            &ctx.accounts.seller_token_account.key(),
            &vote_buy.key(),
            &[],
            payment_to_user,
        )?;
        let (_, bump) = Pubkey::find_program_address(
            &[
                b"vote-buy".as_ref(),
                epoch.to_le_bytes().as_ref(),
                ctx.accounts.config.key().as_ref(),
                ctx.accounts.gauge.key().as_ref(),
            ],
            ctx.program_id,
        );
        invoke_signed(
            &transfer_ix,
            &[
                ctx.accounts.token_vault.to_account_info(),
                ctx.accounts.seller_token_account.to_account_info(),
                ctx.accounts.vote_buy.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
            ],
            &[
                &[
                    b"vote-buy".as_ref(),
                    epoch.to_le_bytes().as_ref(),
                    ctx.accounts.config.key().as_ref(),
                    ctx.accounts.gauge.key().as_ref(),
                    &[bump],
                ],
            ],
        )?;
        let mut data: Vec<u8> = solana_program::hash::hash(
                b"global:close_epoch_gauge_vote",
            )
            .to_bytes()[..8]
            .to_vec();
        data.extend_from_slice(&epoch.to_le_bytes());
        let (_, vote_delegate_bump) = Pubkey::find_program_address(
            &[b"vote-delegate".as_ref(), ctx.accounts.config.key().as_ref()],
            ctx.program_id,
        );
        let close_ix = anchor_lang::solana_program::instruction::Instruction {
            program_id: ctx.accounts.gauge_program.key(),
            accounts: <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    AccountMeta::new(ctx.accounts.epoch_gauge_vote.key(), false),
                    AccountMeta::new_readonly(ctx.accounts.gaugemeister.key(), false),
                    AccountMeta::new_readonly(ctx.accounts.gauge.key(), false),
                    AccountMeta::new_readonly(ctx.accounts.gauge_voter.key(), false),
                    AccountMeta::new_readonly(ctx.accounts.gauge_vote.key(), false),
                    AccountMeta::new_readonly(ctx.accounts.escrow.key(), false),
                    AccountMeta::new_readonly(ctx.accounts.vote_delegate.key(), true),
                    AccountMeta::new(ctx.accounts.vote_delegate.key(), false),
                ]),
            ),
            data,
        };
        invoke_signed(
            &close_ix,
            &[
                ctx.accounts.epoch_gauge_vote.to_account_info(),
                ctx.accounts.gaugemeister.to_account_info(),
                ctx.accounts.gauge.to_account_info(),
                ctx.accounts.gauge_voter.to_account_info(),
                ctx.accounts.gauge_vote.to_account_info(),
                ctx.accounts.escrow.to_account_info(),
                ctx.accounts.vote_delegate.to_account_info(),
                ctx.accounts.vote_delegate.to_account_info(),
            ],
            &[
                &[
                    b"vote-delegate".as_ref(),
                    ctx.accounts.config.key().as_ref(),
                    &[vote_delegate_bump],
                ],
            ],
        )?;
        Ok(())
    }
    pub fn claim_rewards_as_vote_seller(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
    pub fn claim_rewards_as_vote_buyer(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
    pub fn vote(ctx: Context<Vote>, weight: u32) -> Result<()> {
        let mut data: Vec<u8> = solana_program::hash::hash(b"global:gauge_set_vote")
            .to_bytes()[..8]
            .to_vec();
        data.extend_from_slice(weight.to_le_bytes().as_ref());
        let set_weight_ix = solana_program::instruction::Instruction {
            program_id: gauge_state::id(),
            accounts: <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    AccountMeta::new_readonly(ctx.accounts.gaugemeister.key(), false),
                    AccountMeta::new_readonly(ctx.accounts.gauge.key(), false),
                    AccountMeta::new(ctx.accounts.gauge_voter.key(), false),
                    AccountMeta::new(ctx.accounts.gauge_vote.key(), false),
                    AccountMeta::new_readonly(ctx.accounts.escrow.key(), false),
                    AccountMeta::new(ctx.accounts.vote_delegate.key(), true),
                ]),
            ),
            data,
        };
        let (expected_vote_delegate, bump) = Pubkey::find_program_address(
            &[b"vote-delegate".as_ref(), ctx.accounts.config.key().as_ref()],
            ctx.program_id,
        );
        if expected_vote_delegate != ctx.accounts.vote_delegate.key() {
            return Err(ProgramError::InvalidSeeds.into());
        }
        invoke_signed(
            &set_weight_ix,
            &[
                ctx.accounts.gaugemeister.to_account_info(),
                ctx.accounts.gauge.to_account_info(),
                ctx.accounts.gauge_voter.to_account_info(),
                ctx.accounts.gauge_vote.to_account_info(),
                ctx.accounts.escrow.to_account_info(),
                ctx.accounts.vote_delegate.to_account_info(),
            ],
            &[&[b"vote-delegate".as_ref(), ctx.accounts.config.key().as_ref(), &[bump]]],
        )?;
        Ok(())
    }
    #[allow(unused_variables)]
    pub fn set_max_amount(
        ctx: Context<SetMaxAmount>,
        epoch: u32,
        max_amount: u64,
    ) -> Result<()> {
        ctx.accounts.vote_buy.max_amount = Some(max_amount);
        Ok(())
    }
}
/// An Anchor generated module containing the program's set of
/// instructions, where each method handler in the `#[program]` mod is
/// associated with a struct defining the input arguments to the
/// method. These should be used directly, when one wants to serialize
/// Anchor instruction data, for example, when speciying
/// instructions on a client.
pub mod instruction {
    use super::*;
    /// Instruction.
    pub struct CreateConfig {
        pub mints: Vec<Pubkey>,
        pub efficiency_ratio: u64,
        pub script_authority: Pubkey,
    }
    impl borsh::ser::BorshSerialize for CreateConfig
    where
        Vec<Pubkey>: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.mints, writer)?;
            borsh::BorshSerialize::serialize(&self.efficiency_ratio, writer)?;
            borsh::BorshSerialize::serialize(&self.script_authority, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for CreateConfig
    where
        Vec<Pubkey>: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                mints: borsh::BorshDeserialize::deserialize_reader(reader)?,
                efficiency_ratio: borsh::BorshDeserialize::deserialize_reader(reader)?,
                script_authority: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl anchor_lang::Discriminator for CreateConfig {
        const DISCRIMINATOR: [u8; 8] = [201, 207, 243, 114, 75, 111, 47, 189];
    }
    impl anchor_lang::InstructionData for CreateConfig {}
    impl anchor_lang::Owner for CreateConfig {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct UpdateAdmin {
        pub admin: Pubkey,
    }
    impl borsh::ser::BorshSerialize for UpdateAdmin
    where
        Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.admin, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for UpdateAdmin
    where
        Pubkey: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                admin: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl anchor_lang::Discriminator for UpdateAdmin {
        const DISCRIMINATOR: [u8; 8] = [161, 176, 40, 213, 60, 184, 179, 228];
    }
    impl anchor_lang::InstructionData for UpdateAdmin {}
    impl anchor_lang::Owner for UpdateAdmin {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct UpdateScriptAuthority {
        pub script_authority: Pubkey,
    }
    impl borsh::ser::BorshSerialize for UpdateScriptAuthority
    where
        Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.script_authority, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for UpdateScriptAuthority
    where
        Pubkey: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                script_authority: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl anchor_lang::Discriminator for UpdateScriptAuthority {
        const DISCRIMINATOR: [u8; 8] = [57, 237, 52, 47, 112, 6, 101, 72];
    }
    impl anchor_lang::InstructionData for UpdateScriptAuthority {}
    impl anchor_lang::Owner for UpdateScriptAuthority {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct UpdateAllowedMints {
        pub allowed_mints: Vec<Pubkey>,
    }
    impl borsh::ser::BorshSerialize for UpdateAllowedMints
    where
        Vec<Pubkey>: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.allowed_mints, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for UpdateAllowedMints
    where
        Vec<Pubkey>: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                allowed_mints: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl anchor_lang::Discriminator for UpdateAllowedMints {
        const DISCRIMINATOR: [u8; 8] = [146, 211, 161, 2, 10, 20, 94, 67];
    }
    impl anchor_lang::InstructionData for UpdateAllowedMints {}
    impl anchor_lang::Owner for UpdateAllowedMints {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct IncreaseVoteBuy {
        pub epoch: u32,
        pub amount: u64,
    }
    impl borsh::ser::BorshSerialize for IncreaseVoteBuy
    where
        u32: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.epoch, writer)?;
            borsh::BorshSerialize::serialize(&self.amount, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for IncreaseVoteBuy
    where
        u32: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                epoch: borsh::BorshDeserialize::deserialize_reader(reader)?,
                amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl anchor_lang::Discriminator for IncreaseVoteBuy {
        const DISCRIMINATOR: [u8; 8] = [77, 209, 215, 129, 60, 249, 205, 53];
    }
    impl anchor_lang::InstructionData for IncreaseVoteBuy {}
    impl anchor_lang::Owner for IncreaseVoteBuy {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct ClaimVotePayment {
        pub epoch: u32,
    }
    impl borsh::ser::BorshSerialize for ClaimVotePayment
    where
        u32: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.epoch, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for ClaimVotePayment
    where
        u32: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                epoch: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl anchor_lang::Discriminator for ClaimVotePayment {
        const DISCRIMINATOR: [u8; 8] = [130, 32, 214, 224, 134, 130, 22, 171];
    }
    impl anchor_lang::InstructionData for ClaimVotePayment {}
    impl anchor_lang::Owner for ClaimVotePayment {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct ClaimRewardsAsVoteSeller;
    impl borsh::ser::BorshSerialize for ClaimRewardsAsVoteSeller {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for ClaimRewardsAsVoteSeller {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::Discriminator for ClaimRewardsAsVoteSeller {
        const DISCRIMINATOR: [u8; 8] = [168, 111, 100, 156, 51, 232, 126, 155];
    }
    impl anchor_lang::InstructionData for ClaimRewardsAsVoteSeller {}
    impl anchor_lang::Owner for ClaimRewardsAsVoteSeller {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct ClaimRewardsAsVoteBuyer;
    impl borsh::ser::BorshSerialize for ClaimRewardsAsVoteBuyer {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for ClaimRewardsAsVoteBuyer {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::Discriminator for ClaimRewardsAsVoteBuyer {
        const DISCRIMINATOR: [u8; 8] = [224, 33, 221, 154, 35, 107, 212, 36];
    }
    impl anchor_lang::InstructionData for ClaimRewardsAsVoteBuyer {}
    impl anchor_lang::Owner for ClaimRewardsAsVoteBuyer {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct Vote {
        pub weight: u32,
    }
    impl borsh::ser::BorshSerialize for Vote
    where
        u32: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.weight, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Vote
    where
        u32: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                weight: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl anchor_lang::Discriminator for Vote {
        const DISCRIMINATOR: [u8; 8] = [227, 110, 155, 23, 136, 126, 172, 25];
    }
    impl anchor_lang::InstructionData for Vote {}
    impl anchor_lang::Owner for Vote {
        fn owner() -> Pubkey {
            ID
        }
    }
    /// Instruction.
    pub struct SetMaxAmount {
        pub epoch: u32,
        pub max_amount: u64,
    }
    impl borsh::ser::BorshSerialize for SetMaxAmount
    where
        u32: borsh::ser::BorshSerialize,
        u64: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.epoch, writer)?;
            borsh::BorshSerialize::serialize(&self.max_amount, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for SetMaxAmount
    where
        u32: borsh::BorshDeserialize,
        u64: borsh::BorshDeserialize,
    {
        fn deserialize_reader<R: borsh::maybestd::io::Read>(
            reader: &mut R,
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                epoch: borsh::BorshDeserialize::deserialize_reader(reader)?,
                max_amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
            })
        }
    }
    impl anchor_lang::Discriminator for SetMaxAmount {
        const DISCRIMINATOR: [u8; 8] = [226, 133, 252, 54, 185, 15, 122, 154];
    }
    impl anchor_lang::InstructionData for SetMaxAmount {}
    impl anchor_lang::Owner for SetMaxAmount {
        fn owner() -> Pubkey {
            ID
        }
    }
}
/// An Anchor generated module, providing a set of structs
/// mirroring the structs deriving `Accounts`, where each field is
/// a `Pubkey`. This is useful for specifying accounts for a client.
pub mod accounts {
    pub use crate::__client_accounts_update_allowed_mints::*;
    pub use crate::__client_accounts_increase_vote_buy::*;
    pub use crate::__client_accounts_update_script_authority::*;
    pub use crate::__client_accounts_update_admin::*;
    pub use crate::__client_accounts_initialize::*;
    pub use crate::__client_accounts_set_max_amount::*;
    pub use crate::__client_accounts_vote::*;
    pub use crate::__client_accounts_claim_vote_payment::*;
    pub use crate::__client_accounts_create_config::*;
}
pub struct Initialize {}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info, InitializeBumps> for Initialize {
    #[inline(never)]
    fn try_accounts(
        __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
            'info,
        >],
        __ix_data: &[u8],
        __bumps: &mut InitializeBumps,
        __reallocs: &mut std::collections::BTreeSet<
            anchor_lang::solana_program::pubkey::Pubkey,
        >,
    ) -> anchor_lang::Result<Self> {
        Ok(Initialize {})
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for Initialize {
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos
    }
}
#[automatically_derived]
impl anchor_lang::ToAccountMetas for Initialize {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for Initialize {
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::Result<()> {
        Ok(())
    }
}
pub struct InitializeBumps {}
#[automatically_derived]
impl ::core::fmt::Debug for InitializeBumps {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "InitializeBumps")
    }
}
impl Default for InitializeBumps {
    fn default() -> Self {
        InitializeBumps {}
    }
}
impl<'info> anchor_lang::Bumps for Initialize {
    type Bumps = InitializeBumps;
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_initialize {
    use super::*;
    use anchor_lang::prelude::borsh;
    /// Generated client accounts for [`Initialize`].
    pub struct Initialize {}
    impl borsh::ser::BorshSerialize for Initialize {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for Initialize {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// [`cpi::accounts`] module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_initialize {
    use super::*;
    /// Generated CPI struct of the accounts for [`Initialize`].
    pub struct Initialize {}
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for Initialize {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for Initialize {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos
        }
    }
}
#[instruction(mints:Vec<Pubkey>)]
pub struct CreateConfig<'info> {
    #[account(init, payer = payer, space = VoteMarketConfig::LEN)]
    pub config: Account<'info, VoteMarketConfig>,
    pub gaugemeister: Account<'info, gauge_state::Gaugemeister>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = AllowedMints::len(mints.len()),
        seeds = [b"allow-list".as_ref(),
        config.to_account_info().key.as_ref()],
        bump
    )]
    pub allowed_mints: Account<'info, AllowedMints>,
    pub system_program: Program<'info, System>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info, CreateConfigBumps> for CreateConfig<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
            'info,
        >],
        __ix_data: &[u8],
        __bumps: &mut CreateConfigBumps,
        __reallocs: &mut std::collections::BTreeSet<
            anchor_lang::solana_program::pubkey::Pubkey,
        >,
    ) -> anchor_lang::Result<Self> {
        let mut __ix_data = __ix_data;
        struct __Args {
            mints: Vec<Pubkey>,
        }
        impl borsh::ser::BorshSerialize for __Args
        where
            Vec<Pubkey>: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.mints, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for __Args
        where
            Vec<Pubkey>: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    mints: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        let __Args { mints } = __Args::deserialize(&mut __ix_data)
            .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
        if __accounts.is_empty() {
            return Err(anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into());
        }
        let config = &__accounts[0];
        *__accounts = &__accounts[1..];
        let gaugemeister: anchor_lang::accounts::account::Account<
            gauge_state::Gaugemeister,
        > = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("gaugemeister"))?;
        let payer: Signer = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("payer"))?;
        if __accounts.is_empty() {
            return Err(anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into());
        }
        let allowed_mints = &__accounts[0];
        *__accounts = &__accounts[1..];
        let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("system_program"))?;
        let __anchor_rent = Rent::get()?;
        let config = {
            let actual_field = AsRef::<AccountInfo>::as_ref(&config);
            let actual_owner = actual_field.owner;
            let space = VoteMarketConfig::LEN;
            let pa: anchor_lang::accounts::account::Account<VoteMarketConfig> = if !false
                || actual_owner == &anchor_lang::solana_program::system_program::ID
            {
                let __current_lamports = config.lamports();
                if __current_lamports == 0 {
                    let space = space;
                    let lamports = __anchor_rent.minimum_balance(space);
                    let cpi_accounts = anchor_lang::system_program::CreateAccount {
                        from: payer.to_account_info(),
                        to: config.to_account_info(),
                    };
                    let cpi_context = anchor_lang::context::CpiContext::new(
                        system_program.to_account_info(),
                        cpi_accounts,
                    );
                    anchor_lang::system_program::create_account(
                        cpi_context.with_signer(&[]),
                        lamports,
                        space as u64,
                        __program_id,
                    )?;
                } else {
                    if payer.key() == config.key() {
                        return Err(
                            anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                    error_name: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                        .name(),
                                    error_code_number: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                        .into(),
                                    error_msg: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                        .to_string(),
                                    error_origin: Some(
                                        anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                            filename: "programs/vote-market/src/lib.rs",
                                            line: 297u32,
                                        }),
                                    ),
                                    compared_values: None,
                                })
                                .with_pubkeys((payer.key(), config.key())),
                        );
                    }
                    let required_lamports = __anchor_rent
                        .minimum_balance(space)
                        .max(1)
                        .saturating_sub(__current_lamports);
                    if required_lamports > 0 {
                        let cpi_accounts = anchor_lang::system_program::Transfer {
                            from: payer.to_account_info(),
                            to: config.to_account_info(),
                        };
                        let cpi_context = anchor_lang::context::CpiContext::new(
                            system_program.to_account_info(),
                            cpi_accounts,
                        );
                        anchor_lang::system_program::transfer(
                            cpi_context,
                            required_lamports,
                        )?;
                    }
                    let cpi_accounts = anchor_lang::system_program::Allocate {
                        account_to_allocate: config.to_account_info(),
                    };
                    let cpi_context = anchor_lang::context::CpiContext::new(
                        system_program.to_account_info(),
                        cpi_accounts,
                    );
                    anchor_lang::system_program::allocate(
                        cpi_context.with_signer(&[]),
                        space as u64,
                    )?;
                    let cpi_accounts = anchor_lang::system_program::Assign {
                        account_to_assign: config.to_account_info(),
                    };
                    let cpi_context = anchor_lang::context::CpiContext::new(
                        system_program.to_account_info(),
                        cpi_accounts,
                    );
                    anchor_lang::system_program::assign(
                        cpi_context.with_signer(&[]),
                        __program_id,
                    )?;
                }
                match anchor_lang::accounts::account::Account::try_from_unchecked(
                    &config,
                ) {
                    Ok(val) => val,
                    Err(e) => return Err(e.with_account_name("config")),
                }
            } else {
                match anchor_lang::accounts::account::Account::try_from(&config) {
                    Ok(val) => val,
                    Err(e) => return Err(e.with_account_name("config")),
                }
            };
            if false {
                if space != actual_field.data_len() {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSpace,
                            )
                            .with_account_name("config")
                            .with_values((space, actual_field.data_len())),
                    );
                }
                if actual_owner != __program_id {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintOwner,
                            )
                            .with_account_name("config")
                            .with_pubkeys((*actual_owner, *__program_id)),
                    );
                }
                {
                    let required_lamports = __anchor_rent.minimum_balance(space);
                    if pa.to_account_info().lamports() < required_lamports {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintRentExempt,
                                )
                                .with_account_name("config"),
                        );
                    }
                }
            }
            pa
        };
        if !AsRef::<AccountInfo>::as_ref(&config).is_writable {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("config"),
            );
        }
        if !AsRef::<AccountInfo>::as_ref(&config).is_signer {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSigner,
                    )
                    .with_account_name("config"),
            );
        }
        if !__anchor_rent
            .is_exempt(
                config.to_account_info().lamports(),
                config.to_account_info().try_data_len()?,
            )
        {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRentExempt,
                    )
                    .with_account_name("config"),
            );
        }
        let __anchor_rent = Rent::get()?;
        let (__pda_address, __bump) = Pubkey::find_program_address(
            &[b"allow-list".as_ref(), config.to_account_info().key.as_ref()],
            __program_id,
        );
        __bumps.allowed_mints = __bump;
        if allowed_mints.key() != __pda_address {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("allowed_mints")
                    .with_pubkeys((allowed_mints.key(), __pda_address)),
            );
        }
        let allowed_mints = {
            let actual_field = AsRef::<AccountInfo>::as_ref(&allowed_mints);
            let actual_owner = actual_field.owner;
            let space = AllowedMints::len(mints.len());
            let pa: anchor_lang::accounts::account::Account<AllowedMints> = if !false
                || actual_owner == &anchor_lang::solana_program::system_program::ID
            {
                let __current_lamports = allowed_mints.lamports();
                if __current_lamports == 0 {
                    let space = space;
                    let lamports = __anchor_rent.minimum_balance(space);
                    let cpi_accounts = anchor_lang::system_program::CreateAccount {
                        from: payer.to_account_info(),
                        to: allowed_mints.to_account_info(),
                    };
                    let cpi_context = anchor_lang::context::CpiContext::new(
                        system_program.to_account_info(),
                        cpi_accounts,
                    );
                    anchor_lang::system_program::create_account(
                        cpi_context
                            .with_signer(
                                &[
                                    &[
                                        b"allow-list".as_ref(),
                                        config.to_account_info().key.as_ref(),
                                        &[__bump][..],
                                    ][..],
                                ],
                            ),
                        lamports,
                        space as u64,
                        __program_id,
                    )?;
                } else {
                    if payer.key() == allowed_mints.key() {
                        return Err(
                            anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                    error_name: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                        .name(),
                                    error_code_number: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                        .into(),
                                    error_msg: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                        .to_string(),
                                    error_origin: Some(
                                        anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                            filename: "programs/vote-market/src/lib.rs",
                                            line: 297u32,
                                        }),
                                    ),
                                    compared_values: None,
                                })
                                .with_pubkeys((payer.key(), allowed_mints.key())),
                        );
                    }
                    let required_lamports = __anchor_rent
                        .minimum_balance(space)
                        .max(1)
                        .saturating_sub(__current_lamports);
                    if required_lamports > 0 {
                        let cpi_accounts = anchor_lang::system_program::Transfer {
                            from: payer.to_account_info(),
                            to: allowed_mints.to_account_info(),
                        };
                        let cpi_context = anchor_lang::context::CpiContext::new(
                            system_program.to_account_info(),
                            cpi_accounts,
                        );
                        anchor_lang::system_program::transfer(
                            cpi_context,
                            required_lamports,
                        )?;
                    }
                    let cpi_accounts = anchor_lang::system_program::Allocate {
                        account_to_allocate: allowed_mints.to_account_info(),
                    };
                    let cpi_context = anchor_lang::context::CpiContext::new(
                        system_program.to_account_info(),
                        cpi_accounts,
                    );
                    anchor_lang::system_program::allocate(
                        cpi_context
                            .with_signer(
                                &[
                                    &[
                                        b"allow-list".as_ref(),
                                        config.to_account_info().key.as_ref(),
                                        &[__bump][..],
                                    ][..],
                                ],
                            ),
                        space as u64,
                    )?;
                    let cpi_accounts = anchor_lang::system_program::Assign {
                        account_to_assign: allowed_mints.to_account_info(),
                    };
                    let cpi_context = anchor_lang::context::CpiContext::new(
                        system_program.to_account_info(),
                        cpi_accounts,
                    );
                    anchor_lang::system_program::assign(
                        cpi_context
                            .with_signer(
                                &[
                                    &[
                                        b"allow-list".as_ref(),
                                        config.to_account_info().key.as_ref(),
                                        &[__bump][..],
                                    ][..],
                                ],
                            ),
                        __program_id,
                    )?;
                }
                match anchor_lang::accounts::account::Account::try_from_unchecked(
                    &allowed_mints,
                ) {
                    Ok(val) => val,
                    Err(e) => return Err(e.with_account_name("allowed_mints")),
                }
            } else {
                match anchor_lang::accounts::account::Account::try_from(&allowed_mints) {
                    Ok(val) => val,
                    Err(e) => return Err(e.with_account_name("allowed_mints")),
                }
            };
            if false {
                if space != actual_field.data_len() {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSpace,
                            )
                            .with_account_name("allowed_mints")
                            .with_values((space, actual_field.data_len())),
                    );
                }
                if actual_owner != __program_id {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintOwner,
                            )
                            .with_account_name("allowed_mints")
                            .with_pubkeys((*actual_owner, *__program_id)),
                    );
                }
                {
                    let required_lamports = __anchor_rent.minimum_balance(space);
                    if pa.to_account_info().lamports() < required_lamports {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintRentExempt,
                                )
                                .with_account_name("allowed_mints"),
                        );
                    }
                }
            }
            pa
        };
        if !AsRef::<AccountInfo>::as_ref(&allowed_mints).is_writable {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("allowed_mints"),
            );
        }
        if !__anchor_rent
            .is_exempt(
                allowed_mints.to_account_info().lamports(),
                allowed_mints.to_account_info().try_data_len()?,
            )
        {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRentExempt,
                    )
                    .with_account_name("allowed_mints"),
            );
        }
        if !AsRef::<AccountInfo>::as_ref(&payer).is_writable {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("payer"),
            );
        }
        Ok(CreateConfig {
            config,
            gaugemeister,
            payer,
            allowed_mints,
            system_program,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for CreateConfig<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.config.to_account_infos());
        account_infos.extend(self.gaugemeister.to_account_infos());
        account_infos.extend(self.payer.to_account_infos());
        account_infos.extend(self.allowed_mints.to_account_infos());
        account_infos.extend(self.system_program.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for CreateConfig<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.config.to_account_metas(Some(true)));
        account_metas.extend(self.gaugemeister.to_account_metas(None));
        account_metas.extend(self.payer.to_account_metas(None));
        account_metas.extend(self.allowed_mints.to_account_metas(None));
        account_metas.extend(self.system_program.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for CreateConfig<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::Result<()> {
        anchor_lang::AccountsExit::exit(&self.config, program_id)
            .map_err(|e| e.with_account_name("config"))?;
        anchor_lang::AccountsExit::exit(&self.payer, program_id)
            .map_err(|e| e.with_account_name("payer"))?;
        anchor_lang::AccountsExit::exit(&self.allowed_mints, program_id)
            .map_err(|e| e.with_account_name("allowed_mints"))?;
        Ok(())
    }
}
pub struct CreateConfigBumps {
    pub allowed_mints: u8,
}
#[automatically_derived]
impl ::core::fmt::Debug for CreateConfigBumps {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "CreateConfigBumps",
            "allowed_mints",
            &&self.allowed_mints,
        )
    }
}
impl Default for CreateConfigBumps {
    fn default() -> Self {
        CreateConfigBumps {
            allowed_mints: u8::MAX,
        }
    }
}
impl<'info> anchor_lang::Bumps for CreateConfig<'info>
where
    'info: 'info,
{
    type Bumps = CreateConfigBumps;
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_create_config {
    use super::*;
    use anchor_lang::prelude::borsh;
    /// Generated client accounts for [`CreateConfig`].
    pub struct CreateConfig {
        pub config: Pubkey,
        pub gaugemeister: Pubkey,
        pub payer: Pubkey,
        pub allowed_mints: Pubkey,
        pub system_program: Pubkey,
    }
    impl borsh::ser::BorshSerialize for CreateConfig
    where
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.config, writer)?;
            borsh::BorshSerialize::serialize(&self.gaugemeister, writer)?;
            borsh::BorshSerialize::serialize(&self.payer, writer)?;
            borsh::BorshSerialize::serialize(&self.allowed_mints, writer)?;
            borsh::BorshSerialize::serialize(&self.system_program, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for CreateConfig {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.config,
                        true,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.gaugemeister,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.payer,
                        true,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.allowed_mints,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.system_program,
                        false,
                    ),
                );
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// [`cpi::accounts`] module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_create_config {
    use super::*;
    /// Generated CPI struct of the accounts for [`CreateConfig`].
    pub struct CreateConfig<'info> {
        pub config: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub gaugemeister: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub payer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub allowed_mints: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
            'info,
        >,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for CreateConfig<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.config),
                        true,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.gaugemeister),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.payer),
                        true,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.allowed_mints),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.system_program),
                        false,
                    ),
                );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for CreateConfig<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos
                .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.config));
            account_infos
                .extend(
                    anchor_lang::ToAccountInfos::to_account_infos(&self.gaugemeister),
                );
            account_infos
                .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.payer));
            account_infos
                .extend(
                    anchor_lang::ToAccountInfos::to_account_infos(&self.allowed_mints),
                );
            account_infos
                .extend(
                    anchor_lang::ToAccountInfos::to_account_infos(&self.system_program),
                );
            account_infos
        }
    }
}
pub struct UpdateScriptAuthority<'info> {
    #[account(mut, has_one = admin)]
    pub config: Account<'info, VoteMarketConfig>,
    pub admin: Signer<'info>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info, UpdateScriptAuthorityBumps>
for UpdateScriptAuthority<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
            'info,
        >],
        __ix_data: &[u8],
        __bumps: &mut UpdateScriptAuthorityBumps,
        __reallocs: &mut std::collections::BTreeSet<
            anchor_lang::solana_program::pubkey::Pubkey,
        >,
    ) -> anchor_lang::Result<Self> {
        let config: anchor_lang::accounts::account::Account<VoteMarketConfig> = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("config"))?;
        let admin: Signer = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("admin"))?;
        if !AsRef::<AccountInfo>::as_ref(&config).is_writable {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("config"),
            );
        }
        {
            let my_key = config.admin;
            let target_key = admin.key();
            if my_key != target_key {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintHasOne,
                        )
                        .with_account_name("config")
                        .with_pubkeys((my_key, target_key)),
                );
            }
        }
        Ok(UpdateScriptAuthority {
            config,
            admin,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for UpdateScriptAuthority<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.config.to_account_infos());
        account_infos.extend(self.admin.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for UpdateScriptAuthority<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.config.to_account_metas(None));
        account_metas.extend(self.admin.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for UpdateScriptAuthority<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::Result<()> {
        anchor_lang::AccountsExit::exit(&self.config, program_id)
            .map_err(|e| e.with_account_name("config"))?;
        Ok(())
    }
}
pub struct UpdateScriptAuthorityBumps {}
#[automatically_derived]
impl ::core::fmt::Debug for UpdateScriptAuthorityBumps {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "UpdateScriptAuthorityBumps")
    }
}
impl Default for UpdateScriptAuthorityBumps {
    fn default() -> Self {
        UpdateScriptAuthorityBumps {}
    }
}
impl<'info> anchor_lang::Bumps for UpdateScriptAuthority<'info>
where
    'info: 'info,
{
    type Bumps = UpdateScriptAuthorityBumps;
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_update_script_authority {
    use super::*;
    use anchor_lang::prelude::borsh;
    /// Generated client accounts for [`UpdateScriptAuthority`].
    pub struct UpdateScriptAuthority {
        pub config: Pubkey,
        pub admin: Pubkey,
    }
    impl borsh::ser::BorshSerialize for UpdateScriptAuthority
    where
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.config, writer)?;
            borsh::BorshSerialize::serialize(&self.admin, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for UpdateScriptAuthority {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.config,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.admin,
                        true,
                    ),
                );
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// [`cpi::accounts`] module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_update_script_authority {
    use super::*;
    /// Generated CPI struct of the accounts for [`UpdateScriptAuthority`].
    pub struct UpdateScriptAuthority<'info> {
        pub config: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub admin: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for UpdateScriptAuthority<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.config),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.admin),
                        true,
                    ),
                );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for UpdateScriptAuthority<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos
                .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.config));
            account_infos
                .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.admin));
            account_infos
        }
    }
}
pub struct UpdateAdmin<'info> {
    #[account(mut, has_one = admin)]
    pub config: Account<'info, VoteMarketConfig>,
    pub admin: Signer<'info>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info, UpdateAdminBumps> for UpdateAdmin<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
            'info,
        >],
        __ix_data: &[u8],
        __bumps: &mut UpdateAdminBumps,
        __reallocs: &mut std::collections::BTreeSet<
            anchor_lang::solana_program::pubkey::Pubkey,
        >,
    ) -> anchor_lang::Result<Self> {
        let config: anchor_lang::accounts::account::Account<VoteMarketConfig> = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("config"))?;
        let admin: Signer = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("admin"))?;
        if !AsRef::<AccountInfo>::as_ref(&config).is_writable {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("config"),
            );
        }
        {
            let my_key = config.admin;
            let target_key = admin.key();
            if my_key != target_key {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintHasOne,
                        )
                        .with_account_name("config")
                        .with_pubkeys((my_key, target_key)),
                );
            }
        }
        Ok(UpdateAdmin { config, admin })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for UpdateAdmin<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.config.to_account_infos());
        account_infos.extend(self.admin.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for UpdateAdmin<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.config.to_account_metas(None));
        account_metas.extend(self.admin.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for UpdateAdmin<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::Result<()> {
        anchor_lang::AccountsExit::exit(&self.config, program_id)
            .map_err(|e| e.with_account_name("config"))?;
        Ok(())
    }
}
pub struct UpdateAdminBumps {}
#[automatically_derived]
impl ::core::fmt::Debug for UpdateAdminBumps {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "UpdateAdminBumps")
    }
}
impl Default for UpdateAdminBumps {
    fn default() -> Self {
        UpdateAdminBumps {}
    }
}
impl<'info> anchor_lang::Bumps for UpdateAdmin<'info>
where
    'info: 'info,
{
    type Bumps = UpdateAdminBumps;
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_update_admin {
    use super::*;
    use anchor_lang::prelude::borsh;
    /// Generated client accounts for [`UpdateAdmin`].
    pub struct UpdateAdmin {
        pub config: Pubkey,
        pub admin: Pubkey,
    }
    impl borsh::ser::BorshSerialize for UpdateAdmin
    where
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.config, writer)?;
            borsh::BorshSerialize::serialize(&self.admin, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for UpdateAdmin {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.config,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.admin,
                        true,
                    ),
                );
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// [`cpi::accounts`] module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_update_admin {
    use super::*;
    /// Generated CPI struct of the accounts for [`UpdateAdmin`].
    pub struct UpdateAdmin<'info> {
        pub config: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub admin: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for UpdateAdmin<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.config),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.admin),
                        true,
                    ),
                );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for UpdateAdmin<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos
                .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.config));
            account_infos
                .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.admin));
            account_infos
        }
    }
}
pub struct UpdateAllowedMints<'info> {
    #[account(mut, has_one = admin)]
    pub config: Account<'info, VoteMarketConfig>,
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [b"allow-list".as_ref(),
        config.to_account_info().key.as_ref()],
        bump
    )]
    pub allowed_mints: Account<'info, AllowedMints>,
    pub system_program: Program<'info, System>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info, UpdateAllowedMintsBumps>
for UpdateAllowedMints<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
            'info,
        >],
        __ix_data: &[u8],
        __bumps: &mut UpdateAllowedMintsBumps,
        __reallocs: &mut std::collections::BTreeSet<
            anchor_lang::solana_program::pubkey::Pubkey,
        >,
    ) -> anchor_lang::Result<Self> {
        let config: anchor_lang::accounts::account::Account<VoteMarketConfig> = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("config"))?;
        let admin: Signer = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("admin"))?;
        let allowed_mints: anchor_lang::accounts::account::Account<AllowedMints> = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("allowed_mints"))?;
        let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("system_program"))?;
        if !AsRef::<AccountInfo>::as_ref(&config).is_writable {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("config"),
            );
        }
        {
            let my_key = config.admin;
            let target_key = admin.key();
            if my_key != target_key {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintHasOne,
                        )
                        .with_account_name("config")
                        .with_pubkeys((my_key, target_key)),
                );
            }
        }
        if !AsRef::<AccountInfo>::as_ref(&admin).is_writable {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("admin"),
            );
        }
        let (__pda_address, __bump) = Pubkey::find_program_address(
            &[b"allow-list".as_ref(), config.to_account_info().key.as_ref()],
            &__program_id,
        );
        __bumps.allowed_mints = __bump;
        if allowed_mints.key() != __pda_address {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("allowed_mints")
                    .with_pubkeys((allowed_mints.key(), __pda_address)),
            );
        }
        if !AsRef::<AccountInfo>::as_ref(&allowed_mints).is_writable {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("allowed_mints"),
            );
        }
        Ok(UpdateAllowedMints {
            config,
            admin,
            allowed_mints,
            system_program,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for UpdateAllowedMints<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.config.to_account_infos());
        account_infos.extend(self.admin.to_account_infos());
        account_infos.extend(self.allowed_mints.to_account_infos());
        account_infos.extend(self.system_program.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for UpdateAllowedMints<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.config.to_account_metas(None));
        account_metas.extend(self.admin.to_account_metas(None));
        account_metas.extend(self.allowed_mints.to_account_metas(None));
        account_metas.extend(self.system_program.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for UpdateAllowedMints<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::Result<()> {
        anchor_lang::AccountsExit::exit(&self.config, program_id)
            .map_err(|e| e.with_account_name("config"))?;
        anchor_lang::AccountsExit::exit(&self.admin, program_id)
            .map_err(|e| e.with_account_name("admin"))?;
        anchor_lang::AccountsExit::exit(&self.allowed_mints, program_id)
            .map_err(|e| e.with_account_name("allowed_mints"))?;
        Ok(())
    }
}
pub struct UpdateAllowedMintsBumps {
    pub allowed_mints: u8,
}
#[automatically_derived]
impl ::core::fmt::Debug for UpdateAllowedMintsBumps {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "UpdateAllowedMintsBumps",
            "allowed_mints",
            &&self.allowed_mints,
        )
    }
}
impl Default for UpdateAllowedMintsBumps {
    fn default() -> Self {
        UpdateAllowedMintsBumps {
            allowed_mints: u8::MAX,
        }
    }
}
impl<'info> anchor_lang::Bumps for UpdateAllowedMints<'info>
where
    'info: 'info,
{
    type Bumps = UpdateAllowedMintsBumps;
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_update_allowed_mints {
    use super::*;
    use anchor_lang::prelude::borsh;
    /// Generated client accounts for [`UpdateAllowedMints`].
    pub struct UpdateAllowedMints {
        pub config: Pubkey,
        pub admin: Pubkey,
        pub allowed_mints: Pubkey,
        pub system_program: Pubkey,
    }
    impl borsh::ser::BorshSerialize for UpdateAllowedMints
    where
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.config, writer)?;
            borsh::BorshSerialize::serialize(&self.admin, writer)?;
            borsh::BorshSerialize::serialize(&self.allowed_mints, writer)?;
            borsh::BorshSerialize::serialize(&self.system_program, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for UpdateAllowedMints {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.config,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.admin,
                        true,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.allowed_mints,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.system_program,
                        false,
                    ),
                );
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// [`cpi::accounts`] module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_update_allowed_mints {
    use super::*;
    /// Generated CPI struct of the accounts for [`UpdateAllowedMints`].
    pub struct UpdateAllowedMints<'info> {
        pub config: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub admin: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub allowed_mints: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
            'info,
        >,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for UpdateAllowedMints<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.config),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.admin),
                        true,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.allowed_mints),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.system_program),
                        false,
                    ),
                );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for UpdateAllowedMints<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos
                .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.config));
            account_infos
                .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.admin));
            account_infos
                .extend(
                    anchor_lang::ToAccountInfos::to_account_infos(&self.allowed_mints),
                );
            account_infos
                .extend(
                    anchor_lang::ToAccountInfos::to_account_infos(&self.system_program),
                );
            account_infos
        }
    }
}
#[instruction(epoch:u32)]
pub struct IncreaseVoteBuy<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut, associated_token::mint = mint, associated_token::authority = buyer)]
    pub buyer_token_account: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = mint,
        associated_token::authority = vote_buy
    )]
    pub token_vault: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    #[account(has_one = gaugemeister)]
    pub config: Account<'info, VoteMarketConfig>,
    pub gaugemeister: Account<'info, gauge_state::Gaugemeister>,
    #[account(
        init_if_needed,
        payer = buyer,
        space = VoteBuy::LEN,
        seeds = [b"vote-buy".as_ref(),
        epoch.to_le_bytes().as_ref(),
        config.key().as_ref(),
        gauge.key().as_ref()],
        bump
    )]
    pub vote_buy: Account<'info, VoteBuy>,
    #[account(has_one = gaugemeister, constraint = !gauge.is_disabled)]
    pub gauge: Account<'info, gauge_state::Gauge>,
    #[account(seeds = [b"allow-list".as_ref(), config.key().as_ref()], bump)]
    pub allowed_mints: Account<'info, AllowedMints>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info, IncreaseVoteBuyBumps> for IncreaseVoteBuy<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
            'info,
        >],
        __ix_data: &[u8],
        __bumps: &mut IncreaseVoteBuyBumps,
        __reallocs: &mut std::collections::BTreeSet<
            anchor_lang::solana_program::pubkey::Pubkey,
        >,
    ) -> anchor_lang::Result<Self> {
        let mut __ix_data = __ix_data;
        struct __Args {
            epoch: u32,
        }
        impl borsh::ser::BorshSerialize for __Args
        where
            u32: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.epoch, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for __Args
        where
            u32: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    epoch: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        let __Args { epoch } = __Args::deserialize(&mut __ix_data)
            .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
        let buyer: Signer = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("buyer"))?;
        let buyer_token_account: anchor_lang::accounts::account::Account<TokenAccount> = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("buyer_token_account"))?;
        if __accounts.is_empty() {
            return Err(anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into());
        }
        let token_vault = &__accounts[0];
        *__accounts = &__accounts[1..];
        let mint: anchor_lang::accounts::account::Account<Mint> = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("mint"))?;
        let config: anchor_lang::accounts::account::Account<VoteMarketConfig> = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("config"))?;
        let gaugemeister: anchor_lang::accounts::account::Account<
            gauge_state::Gaugemeister,
        > = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("gaugemeister"))?;
        if __accounts.is_empty() {
            return Err(anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into());
        }
        let vote_buy = &__accounts[0];
        *__accounts = &__accounts[1..];
        let gauge: anchor_lang::accounts::account::Account<gauge_state::Gauge> = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("gauge"))?;
        let allowed_mints: anchor_lang::accounts::account::Account<AllowedMints> = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("allowed_mints"))?;
        let token_program: anchor_lang::accounts::program::Program<Token> = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("token_program"))?;
        let associated_token_program: anchor_lang::accounts::program::Program<
            AssociatedToken,
        > = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("associated_token_program"))?;
        let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("system_program"))?;
        let __anchor_rent = Rent::get()?;
        let token_vault: anchor_lang::accounts::account::Account<TokenAccount> = {
            let owner_program = AsRef::<AccountInfo>::as_ref(&token_vault).owner;
            if !true || owner_program == &anchor_lang::solana_program::system_program::ID
            {
                let cpi_program = associated_token_program.to_account_info();
                let cpi_accounts = ::anchor_spl::associated_token::Create {
                    payer: buyer.to_account_info(),
                    associated_token: token_vault.to_account_info(),
                    authority: vote_buy.to_account_info(),
                    mint: mint.to_account_info(),
                    system_program: system_program.to_account_info(),
                    token_program: token_program.to_account_info(),
                };
                let cpi_ctx = anchor_lang::context::CpiContext::new(
                    cpi_program,
                    cpi_accounts,
                );
                ::anchor_spl::associated_token::create(cpi_ctx)?;
            }
            let pa: anchor_lang::accounts::account::Account<TokenAccount> = match anchor_lang::accounts::account::Account::try_from_unchecked(
                &token_vault,
            ) {
                Ok(val) => val,
                Err(e) => return Err(e.with_account_name("token_vault")),
            };
            if true {
                if pa.mint != mint.key() {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintTokenMint,
                            )
                            .with_account_name("token_vault")
                            .with_pubkeys((pa.mint, mint.key())),
                    );
                }
                if pa.owner != vote_buy.key() {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintTokenOwner,
                            )
                            .with_account_name("token_vault")
                            .with_pubkeys((pa.owner, vote_buy.key())),
                    );
                }
                if owner_program != &token_program.key() {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintAssociatedTokenTokenProgram,
                            )
                            .with_account_name("token_vault")
                            .with_pubkeys((*owner_program, token_program.key())),
                    );
                }
                if pa.key()
                    != ::anchor_spl::associated_token::get_associated_token_address_with_program_id(
                        &vote_buy.key(),
                        &mint.key(),
                        &token_program.key(),
                    )
                {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::AccountNotAssociatedTokenAccount,
                            )
                            .with_account_name("token_vault"),
                    );
                }
            }
            pa
        };
        if !AsRef::<AccountInfo>::as_ref(&token_vault).is_writable {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("token_vault"),
            );
        }
        if !__anchor_rent
            .is_exempt(
                token_vault.to_account_info().lamports(),
                token_vault.to_account_info().try_data_len()?,
            )
        {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRentExempt,
                    )
                    .with_account_name("token_vault"),
            );
        }
        let __anchor_rent = Rent::get()?;
        let (__pda_address, __bump) = Pubkey::find_program_address(
            &[
                b"vote-buy".as_ref(),
                epoch.to_le_bytes().as_ref(),
                config.key().as_ref(),
                gauge.key().as_ref(),
            ],
            __program_id,
        );
        __bumps.vote_buy = __bump;
        if vote_buy.key() != __pda_address {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("vote_buy")
                    .with_pubkeys((vote_buy.key(), __pda_address)),
            );
        }
        let vote_buy = {
            let actual_field = AsRef::<AccountInfo>::as_ref(&vote_buy);
            let actual_owner = actual_field.owner;
            let space = VoteBuy::LEN;
            let pa: anchor_lang::accounts::account::Account<VoteBuy> = if !true
                || actual_owner == &anchor_lang::solana_program::system_program::ID
            {
                let __current_lamports = vote_buy.lamports();
                if __current_lamports == 0 {
                    let space = space;
                    let lamports = __anchor_rent.minimum_balance(space);
                    let cpi_accounts = anchor_lang::system_program::CreateAccount {
                        from: buyer.to_account_info(),
                        to: vote_buy.to_account_info(),
                    };
                    let cpi_context = anchor_lang::context::CpiContext::new(
                        system_program.to_account_info(),
                        cpi_accounts,
                    );
                    anchor_lang::system_program::create_account(
                        cpi_context
                            .with_signer(
                                &[
                                    &[
                                        b"vote-buy".as_ref(),
                                        epoch.to_le_bytes().as_ref(),
                                        config.key().as_ref(),
                                        gauge.key().as_ref(),
                                        &[__bump][..],
                                    ][..],
                                ],
                            ),
                        lamports,
                        space as u64,
                        __program_id,
                    )?;
                } else {
                    if buyer.key() == vote_buy.key() {
                        return Err(
                            anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                                    error_name: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                        .name(),
                                    error_code_number: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                        .into(),
                                    error_msg: anchor_lang::error::ErrorCode::TryingToInitPayerAsProgramAccount
                                        .to_string(),
                                    error_origin: Some(
                                        anchor_lang::error::ErrorOrigin::Source(anchor_lang::error::Source {
                                            filename: "programs/vote-market/src/lib.rs",
                                            line: 347u32,
                                        }),
                                    ),
                                    compared_values: None,
                                })
                                .with_pubkeys((buyer.key(), vote_buy.key())),
                        );
                    }
                    let required_lamports = __anchor_rent
                        .minimum_balance(space)
                        .max(1)
                        .saturating_sub(__current_lamports);
                    if required_lamports > 0 {
                        let cpi_accounts = anchor_lang::system_program::Transfer {
                            from: buyer.to_account_info(),
                            to: vote_buy.to_account_info(),
                        };
                        let cpi_context = anchor_lang::context::CpiContext::new(
                            system_program.to_account_info(),
                            cpi_accounts,
                        );
                        anchor_lang::system_program::transfer(
                            cpi_context,
                            required_lamports,
                        )?;
                    }
                    let cpi_accounts = anchor_lang::system_program::Allocate {
                        account_to_allocate: vote_buy.to_account_info(),
                    };
                    let cpi_context = anchor_lang::context::CpiContext::new(
                        system_program.to_account_info(),
                        cpi_accounts,
                    );
                    anchor_lang::system_program::allocate(
                        cpi_context
                            .with_signer(
                                &[
                                    &[
                                        b"vote-buy".as_ref(),
                                        epoch.to_le_bytes().as_ref(),
                                        config.key().as_ref(),
                                        gauge.key().as_ref(),
                                        &[__bump][..],
                                    ][..],
                                ],
                            ),
                        space as u64,
                    )?;
                    let cpi_accounts = anchor_lang::system_program::Assign {
                        account_to_assign: vote_buy.to_account_info(),
                    };
                    let cpi_context = anchor_lang::context::CpiContext::new(
                        system_program.to_account_info(),
                        cpi_accounts,
                    );
                    anchor_lang::system_program::assign(
                        cpi_context
                            .with_signer(
                                &[
                                    &[
                                        b"vote-buy".as_ref(),
                                        epoch.to_le_bytes().as_ref(),
                                        config.key().as_ref(),
                                        gauge.key().as_ref(),
                                        &[__bump][..],
                                    ][..],
                                ],
                            ),
                        __program_id,
                    )?;
                }
                match anchor_lang::accounts::account::Account::try_from_unchecked(
                    &vote_buy,
                ) {
                    Ok(val) => val,
                    Err(e) => return Err(e.with_account_name("vote_buy")),
                }
            } else {
                match anchor_lang::accounts::account::Account::try_from(&vote_buy) {
                    Ok(val) => val,
                    Err(e) => return Err(e.with_account_name("vote_buy")),
                }
            };
            if true {
                if space != actual_field.data_len() {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSpace,
                            )
                            .with_account_name("vote_buy")
                            .with_values((space, actual_field.data_len())),
                    );
                }
                if actual_owner != __program_id {
                    return Err(
                        anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintOwner,
                            )
                            .with_account_name("vote_buy")
                            .with_pubkeys((*actual_owner, *__program_id)),
                    );
                }
                {
                    let required_lamports = __anchor_rent.minimum_balance(space);
                    if pa.to_account_info().lamports() < required_lamports {
                        return Err(
                            anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintRentExempt,
                                )
                                .with_account_name("vote_buy"),
                        );
                    }
                }
            }
            pa
        };
        if !AsRef::<AccountInfo>::as_ref(&vote_buy).is_writable {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vote_buy"),
            );
        }
        if !__anchor_rent
            .is_exempt(
                vote_buy.to_account_info().lamports(),
                vote_buy.to_account_info().try_data_len()?,
            )
        {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRentExempt,
                    )
                    .with_account_name("vote_buy"),
            );
        }
        if !AsRef::<AccountInfo>::as_ref(&buyer).is_writable {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("buyer"),
            );
        }
        {
            let my_owner = buyer_token_account.owner;
            let wallet_address = buyer.key();
            if my_owner != wallet_address {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintTokenOwner,
                        )
                        .with_account_name("buyer_token_account")
                        .with_pubkeys((my_owner, wallet_address)),
                );
            }
            let __associated_token_address = ::anchor_spl::associated_token::get_associated_token_address(
                &wallet_address,
                &mint.key(),
            );
            let my_key = buyer_token_account.key();
            if my_key != __associated_token_address {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintAssociated,
                        )
                        .with_account_name("buyer_token_account")
                        .with_pubkeys((my_key, __associated_token_address)),
                );
            }
        }
        if !AsRef::<AccountInfo>::as_ref(&buyer_token_account).is_writable {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("buyer_token_account"),
            );
        }
        {
            let my_key = config.gaugemeister;
            let target_key = gaugemeister.key();
            if my_key != target_key {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintHasOne,
                        )
                        .with_account_name("config")
                        .with_pubkeys((my_key, target_key)),
                );
            }
        }
        {
            let my_key = gauge.gaugemeister;
            let target_key = gaugemeister.key();
            if my_key != target_key {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintHasOne,
                        )
                        .with_account_name("gauge")
                        .with_pubkeys((my_key, target_key)),
                );
            }
        }
        if !(!gauge.is_disabled) {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRaw,
                    )
                    .with_account_name("gauge"),
            );
        }
        let (__pda_address, __bump) = Pubkey::find_program_address(
            &[b"allow-list".as_ref(), config.key().as_ref()],
            &__program_id,
        );
        __bumps.allowed_mints = __bump;
        if allowed_mints.key() != __pda_address {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("allowed_mints")
                    .with_pubkeys((allowed_mints.key(), __pda_address)),
            );
        }
        Ok(IncreaseVoteBuy {
            buyer,
            buyer_token_account,
            token_vault,
            mint,
            config,
            gaugemeister,
            vote_buy,
            gauge,
            allowed_mints,
            token_program,
            associated_token_program,
            system_program,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for IncreaseVoteBuy<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.buyer.to_account_infos());
        account_infos.extend(self.buyer_token_account.to_account_infos());
        account_infos.extend(self.token_vault.to_account_infos());
        account_infos.extend(self.mint.to_account_infos());
        account_infos.extend(self.config.to_account_infos());
        account_infos.extend(self.gaugemeister.to_account_infos());
        account_infos.extend(self.vote_buy.to_account_infos());
        account_infos.extend(self.gauge.to_account_infos());
        account_infos.extend(self.allowed_mints.to_account_infos());
        account_infos.extend(self.token_program.to_account_infos());
        account_infos.extend(self.associated_token_program.to_account_infos());
        account_infos.extend(self.system_program.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for IncreaseVoteBuy<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.buyer.to_account_metas(None));
        account_metas.extend(self.buyer_token_account.to_account_metas(None));
        account_metas.extend(self.token_vault.to_account_metas(None));
        account_metas.extend(self.mint.to_account_metas(None));
        account_metas.extend(self.config.to_account_metas(None));
        account_metas.extend(self.gaugemeister.to_account_metas(None));
        account_metas.extend(self.vote_buy.to_account_metas(None));
        account_metas.extend(self.gauge.to_account_metas(None));
        account_metas.extend(self.allowed_mints.to_account_metas(None));
        account_metas.extend(self.token_program.to_account_metas(None));
        account_metas.extend(self.associated_token_program.to_account_metas(None));
        account_metas.extend(self.system_program.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for IncreaseVoteBuy<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::Result<()> {
        anchor_lang::AccountsExit::exit(&self.buyer, program_id)
            .map_err(|e| e.with_account_name("buyer"))?;
        anchor_lang::AccountsExit::exit(&self.buyer_token_account, program_id)
            .map_err(|e| e.with_account_name("buyer_token_account"))?;
        anchor_lang::AccountsExit::exit(&self.token_vault, program_id)
            .map_err(|e| e.with_account_name("token_vault"))?;
        anchor_lang::AccountsExit::exit(&self.vote_buy, program_id)
            .map_err(|e| e.with_account_name("vote_buy"))?;
        Ok(())
    }
}
pub struct IncreaseVoteBuyBumps {
    pub vote_buy: u8,
    pub allowed_mints: u8,
}
#[automatically_derived]
impl ::core::fmt::Debug for IncreaseVoteBuyBumps {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "IncreaseVoteBuyBumps",
            "vote_buy",
            &self.vote_buy,
            "allowed_mints",
            &&self.allowed_mints,
        )
    }
}
impl Default for IncreaseVoteBuyBumps {
    fn default() -> Self {
        IncreaseVoteBuyBumps {
            vote_buy: u8::MAX,
            allowed_mints: u8::MAX,
        }
    }
}
impl<'info> anchor_lang::Bumps for IncreaseVoteBuy<'info>
where
    'info: 'info,
{
    type Bumps = IncreaseVoteBuyBumps;
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_increase_vote_buy {
    use super::*;
    use anchor_lang::prelude::borsh;
    /// Generated client accounts for [`IncreaseVoteBuy`].
    pub struct IncreaseVoteBuy {
        pub buyer: Pubkey,
        pub buyer_token_account: Pubkey,
        pub token_vault: Pubkey,
        pub mint: Pubkey,
        pub config: Pubkey,
        pub gaugemeister: Pubkey,
        pub vote_buy: Pubkey,
        pub gauge: Pubkey,
        pub allowed_mints: Pubkey,
        pub token_program: Pubkey,
        pub associated_token_program: Pubkey,
        pub system_program: Pubkey,
    }
    impl borsh::ser::BorshSerialize for IncreaseVoteBuy
    where
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.buyer, writer)?;
            borsh::BorshSerialize::serialize(&self.buyer_token_account, writer)?;
            borsh::BorshSerialize::serialize(&self.token_vault, writer)?;
            borsh::BorshSerialize::serialize(&self.mint, writer)?;
            borsh::BorshSerialize::serialize(&self.config, writer)?;
            borsh::BorshSerialize::serialize(&self.gaugemeister, writer)?;
            borsh::BorshSerialize::serialize(&self.vote_buy, writer)?;
            borsh::BorshSerialize::serialize(&self.gauge, writer)?;
            borsh::BorshSerialize::serialize(&self.allowed_mints, writer)?;
            borsh::BorshSerialize::serialize(&self.token_program, writer)?;
            borsh::BorshSerialize::serialize(&self.associated_token_program, writer)?;
            borsh::BorshSerialize::serialize(&self.system_program, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for IncreaseVoteBuy {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.buyer,
                        true,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.buyer_token_account,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.token_vault,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.mint,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.config,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.gaugemeister,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vote_buy,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.gauge,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.allowed_mints,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.token_program,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.associated_token_program,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.system_program,
                        false,
                    ),
                );
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// [`cpi::accounts`] module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_increase_vote_buy {
    use super::*;
    /// Generated CPI struct of the accounts for [`IncreaseVoteBuy`].
    pub struct IncreaseVoteBuy<'info> {
        pub buyer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub buyer_token_account: anchor_lang::solana_program::account_info::AccountInfo<
            'info,
        >,
        pub token_vault: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub mint: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub config: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub gaugemeister: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub vote_buy: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub gauge: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub allowed_mints: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub associated_token_program: anchor_lang::solana_program::account_info::AccountInfo<
            'info,
        >,
        pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
            'info,
        >,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for IncreaseVoteBuy<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.buyer),
                        true,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.buyer_token_account),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.token_vault),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.mint),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.config),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.gaugemeister),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vote_buy),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.gauge),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.allowed_mints),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.token_program),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.associated_token_program),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.system_program),
                        false,
                    ),
                );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for IncreaseVoteBuy<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos
                .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.buyer));
            account_infos
                .extend(
                    anchor_lang::ToAccountInfos::to_account_infos(
                        &self.buyer_token_account,
                    ),
                );
            account_infos
                .extend(
                    anchor_lang::ToAccountInfos::to_account_infos(&self.token_vault),
                );
            account_infos
                .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.mint));
            account_infos
                .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.config));
            account_infos
                .extend(
                    anchor_lang::ToAccountInfos::to_account_infos(&self.gaugemeister),
                );
            account_infos
                .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.vote_buy));
            account_infos
                .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.gauge));
            account_infos
                .extend(
                    anchor_lang::ToAccountInfos::to_account_infos(&self.allowed_mints),
                );
            account_infos
                .extend(
                    anchor_lang::ToAccountInfos::to_account_infos(&self.token_program),
                );
            account_infos
                .extend(
                    anchor_lang::ToAccountInfos::to_account_infos(
                        &self.associated_token_program,
                    ),
                );
            account_infos
                .extend(
                    anchor_lang::ToAccountInfos::to_account_infos(&self.system_program),
                );
            account_infos
        }
    }
}
#[instruction(epoch:u32)]
pub struct ClaimVotePayment<'info> {
    pub script_authority: Signer<'info>,
    #[account(mut)]
    pub seller: SystemAccount<'info>,
    #[account(mut, associated_token::mint = mint, associated_token::authority = seller)]
    pub seller_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = vote_buy,
    )]
    pub token_vault: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    #[account(has_one = gaugemeister, has_one = script_authority)]
    pub config: Box<Account<'info, VoteMarketConfig>>,
    #[account(
        mut,
        seeds = [b"vote-buy".as_ref(),
        epoch.to_le_bytes().as_ref(),
        config.key().as_ref(),
        gauge.key().as_ref()],
        bump
    )]
    pub vote_buy: Box<Account<'info, VoteBuy>>,
    #[account(mut, seeds = [b"vote-delegate", config.key().as_ref()], bump)]
    pub vote_delegate: SystemAccount<'info>,
    #[account(
        has_one = vote_delegate,
        constraint = escrow.owner = = seller.key(),
        owner = locked_voter_program.key(),
        seeds = [b"Escrow",
        gaugemeister.locker.as_ref(),
        escrow.owner.as_ref()],
        bump,
        seeds::program = locked_voter_state::id()
    )]
    pub escrow: Account<'info, locked_voter_state::Escrow>,
    #[account(
        owner = gauge_program.key(),
        constraint = gaugemeister.locker = = escrow.locker
    )]
    pub gaugemeister: Account<'info, gauge_state::Gaugemeister>,
    #[account(
        has_one = gaugemeister,
        has_one = escrow,
        seeds = [b"GaugeVoter",
        gaugemeister.key().as_ref(),
        escrow.key().as_ref()],
        bump,
        seeds::program = gauge_program.key(),
    )]
    pub gauge_voter: Account<'info, gauge_state::GaugeVoter>,
    #[account(
        has_one = gauge_voter,
        has_one = gauge,
        seeds = [b"GaugeVote",
        gauge_voter.key().as_ref(),
        gauge.key().as_ref()],
        bump,
        seeds::program = gauge_program.key()
    )]
    pub gauge_vote: Account<'info, gauge_state::GaugeVote>,
    #[account(
        has_one = gauge_voter,
        owner = gauge_program.key(),
        seeds = [b"EpochGaugeVoter",
        gauge_voter.key().as_ref(),
        epoch.to_le_bytes().as_ref()],
        bump,
        seeds::program = gauge_program.key(),
    )]
    pub epoch_gauge_voter: Account<'info, gauge_state::EpochGaugeVoter>,
    #[account(has_one = gaugemeister, constraint = !gauge.is_disabled)]
    pub gauge: Account<'info, gauge_state::Gauge>,
    #[account(has_one = gauge, owner = gauge_program.key())]
    pub epoch_gauge: Account<'info, gauge_state::EpochGauge>,
    #[account(mut, owner = gauge_program.key())]
    pub epoch_gauge_vote: Account<'info, gauge_state::EpochGaugeVote>,
    pub gauge_program: Program<'info, GaugeProgram>,
    pub locked_voter_program: Program<'info, LockedVoterProgram>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info, ClaimVotePaymentBumps>
for ClaimVotePayment<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
            'info,
        >],
        __ix_data: &[u8],
        __bumps: &mut ClaimVotePaymentBumps,
        __reallocs: &mut std::collections::BTreeSet<
            anchor_lang::solana_program::pubkey::Pubkey,
        >,
    ) -> anchor_lang::Result<Self> {
        let mut __ix_data = __ix_data;
        struct __Args {
            epoch: u32,
        }
        impl borsh::ser::BorshSerialize for __Args
        where
            u32: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.epoch, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for __Args
        where
            u32: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    epoch: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        let __Args { epoch } = __Args::deserialize(&mut __ix_data)
            .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
        let script_authority: Signer = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("script_authority"))?;
        let seller: SystemAccount = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("seller"))?;
        let seller_token_account: anchor_lang::accounts::account::Account<
            TokenAccount,
        > = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("seller_token_account"))?;
        let token_vault: anchor_lang::accounts::account::Account<TokenAccount> = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("token_vault"))?;
        let mint: anchor_lang::accounts::account::Account<Mint> = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("mint"))?;
        let config: Box<anchor_lang::accounts::account::Account<VoteMarketConfig>> = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("config"))?;
        let vote_buy: Box<anchor_lang::accounts::account::Account<VoteBuy>> = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("vote_buy"))?;
        let vote_delegate: SystemAccount = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("vote_delegate"))?;
        let escrow: anchor_lang::accounts::account::Account<
            locked_voter_state::Escrow,
        > = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("escrow"))?;
        let gaugemeister: anchor_lang::accounts::account::Account<
            gauge_state::Gaugemeister,
        > = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("gaugemeister"))?;
        let gauge_voter: anchor_lang::accounts::account::Account<
            gauge_state::GaugeVoter,
        > = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("gauge_voter"))?;
        let gauge_vote: anchor_lang::accounts::account::Account<
            gauge_state::GaugeVote,
        > = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("gauge_vote"))?;
        let epoch_gauge_voter: anchor_lang::accounts::account::Account<
            gauge_state::EpochGaugeVoter,
        > = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("epoch_gauge_voter"))?;
        let gauge: anchor_lang::accounts::account::Account<gauge_state::Gauge> = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("gauge"))?;
        let epoch_gauge: anchor_lang::accounts::account::Account<
            gauge_state::EpochGauge,
        > = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("epoch_gauge"))?;
        let epoch_gauge_vote: anchor_lang::accounts::account::Account<
            gauge_state::EpochGaugeVote,
        > = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("epoch_gauge_vote"))?;
        let gauge_program: anchor_lang::accounts::program::Program<GaugeProgram> = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("gauge_program"))?;
        let locked_voter_program: anchor_lang::accounts::program::Program<
            LockedVoterProgram,
        > = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("locked_voter_program"))?;
        let token_program: anchor_lang::accounts::program::Program<Token> = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("token_program"))?;
        let system_program: anchor_lang::accounts::program::Program<System> = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("system_program"))?;
        if !AsRef::<AccountInfo>::as_ref(&seller).is_writable {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("seller"),
            );
        }
        {
            let my_owner = seller_token_account.owner;
            let wallet_address = seller.key();
            if my_owner != wallet_address {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintTokenOwner,
                        )
                        .with_account_name("seller_token_account")
                        .with_pubkeys((my_owner, wallet_address)),
                );
            }
            let __associated_token_address = ::anchor_spl::associated_token::get_associated_token_address(
                &wallet_address,
                &mint.key(),
            );
            let my_key = seller_token_account.key();
            if my_key != __associated_token_address {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintAssociated,
                        )
                        .with_account_name("seller_token_account")
                        .with_pubkeys((my_key, __associated_token_address)),
                );
            }
        }
        if !AsRef::<AccountInfo>::as_ref(&seller_token_account).is_writable {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("seller_token_account"),
            );
        }
        {
            let my_owner = token_vault.owner;
            let wallet_address = vote_buy.key();
            if my_owner != wallet_address {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintTokenOwner,
                        )
                        .with_account_name("token_vault")
                        .with_pubkeys((my_owner, wallet_address)),
                );
            }
            let __associated_token_address = ::anchor_spl::associated_token::get_associated_token_address(
                &wallet_address,
                &mint.key(),
            );
            let my_key = token_vault.key();
            if my_key != __associated_token_address {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintAssociated,
                        )
                        .with_account_name("token_vault")
                        .with_pubkeys((my_key, __associated_token_address)),
                );
            }
        }
        if !AsRef::<AccountInfo>::as_ref(&token_vault).is_writable {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("token_vault"),
            );
        }
        {
            let my_key = config.gaugemeister;
            let target_key = gaugemeister.key();
            if my_key != target_key {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintHasOne,
                        )
                        .with_account_name("config")
                        .with_pubkeys((my_key, target_key)),
                );
            }
        }
        {
            let my_key = config.script_authority;
            let target_key = script_authority.key();
            if my_key != target_key {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintHasOne,
                        )
                        .with_account_name("config")
                        .with_pubkeys((my_key, target_key)),
                );
            }
        }
        let (__pda_address, __bump) = Pubkey::find_program_address(
            &[
                b"vote-buy".as_ref(),
                epoch.to_le_bytes().as_ref(),
                config.key().as_ref(),
                gauge.key().as_ref(),
            ],
            &__program_id,
        );
        __bumps.vote_buy = __bump;
        if vote_buy.key() != __pda_address {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("vote_buy")
                    .with_pubkeys((vote_buy.key(), __pda_address)),
            );
        }
        if !AsRef::<AccountInfo>::as_ref(vote_buy.as_ref()).is_writable {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vote_buy"),
            );
        }
        let (__pda_address, __bump) = Pubkey::find_program_address(
            &[b"vote-delegate", config.key().as_ref()],
            &__program_id,
        );
        __bumps.vote_delegate = __bump;
        if vote_delegate.key() != __pda_address {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("vote_delegate")
                    .with_pubkeys((vote_delegate.key(), __pda_address)),
            );
        }
        if !AsRef::<AccountInfo>::as_ref(&vote_delegate).is_writable {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vote_delegate"),
            );
        }
        let (__pda_address, __bump) = Pubkey::find_program_address(
            &[b"Escrow", gaugemeister.locker.as_ref(), escrow.owner.as_ref()],
            &locked_voter_state::id().key(),
        );
        __bumps.escrow = __bump;
        if escrow.key() != __pda_address {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("escrow")
                    .with_pubkeys((escrow.key(), __pda_address)),
            );
        }
        {
            let my_key = escrow.vote_delegate;
            let target_key = vote_delegate.key();
            if my_key != target_key {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintHasOne,
                        )
                        .with_account_name("escrow")
                        .with_pubkeys((my_key, target_key)),
                );
            }
        }
        if !(escrow.owner == seller.key()) {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRaw,
                    )
                    .with_account_name("escrow"),
            );
        }
        {
            let my_owner = AsRef::<AccountInfo>::as_ref(&escrow).owner;
            let owner_address = locked_voter_program.key();
            if my_owner != &owner_address {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintOwner,
                        )
                        .with_account_name("escrow")
                        .with_pubkeys((*my_owner, owner_address)),
                );
            }
        }
        if !(gaugemeister.locker == escrow.locker) {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRaw,
                    )
                    .with_account_name("gaugemeister"),
            );
        }
        {
            let my_owner = AsRef::<AccountInfo>::as_ref(&gaugemeister).owner;
            let owner_address = gauge_program.key();
            if my_owner != &owner_address {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintOwner,
                        )
                        .with_account_name("gaugemeister")
                        .with_pubkeys((*my_owner, owner_address)),
                );
            }
        }
        let (__pda_address, __bump) = Pubkey::find_program_address(
            &[b"GaugeVoter", gaugemeister.key().as_ref(), escrow.key().as_ref()],
            &gauge_program.key().key(),
        );
        __bumps.gauge_voter = __bump;
        if gauge_voter.key() != __pda_address {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("gauge_voter")
                    .with_pubkeys((gauge_voter.key(), __pda_address)),
            );
        }
        {
            let my_key = gauge_voter.gaugemeister;
            let target_key = gaugemeister.key();
            if my_key != target_key {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintHasOne,
                        )
                        .with_account_name("gauge_voter")
                        .with_pubkeys((my_key, target_key)),
                );
            }
        }
        {
            let my_key = gauge_voter.escrow;
            let target_key = escrow.key();
            if my_key != target_key {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintHasOne,
                        )
                        .with_account_name("gauge_voter")
                        .with_pubkeys((my_key, target_key)),
                );
            }
        }
        let (__pda_address, __bump) = Pubkey::find_program_address(
            &[b"GaugeVote", gauge_voter.key().as_ref(), gauge.key().as_ref()],
            &gauge_program.key().key(),
        );
        __bumps.gauge_vote = __bump;
        if gauge_vote.key() != __pda_address {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("gauge_vote")
                    .with_pubkeys((gauge_vote.key(), __pda_address)),
            );
        }
        {
            let my_key = gauge_vote.gauge_voter;
            let target_key = gauge_voter.key();
            if my_key != target_key {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintHasOne,
                        )
                        .with_account_name("gauge_vote")
                        .with_pubkeys((my_key, target_key)),
                );
            }
        }
        {
            let my_key = gauge_vote.gauge;
            let target_key = gauge.key();
            if my_key != target_key {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintHasOne,
                        )
                        .with_account_name("gauge_vote")
                        .with_pubkeys((my_key, target_key)),
                );
            }
        }
        let (__pda_address, __bump) = Pubkey::find_program_address(
            &[
                b"EpochGaugeVoter",
                gauge_voter.key().as_ref(),
                epoch.to_le_bytes().as_ref(),
            ],
            &gauge_program.key().key(),
        );
        __bumps.epoch_gauge_voter = __bump;
        if epoch_gauge_voter.key() != __pda_address {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("epoch_gauge_voter")
                    .with_pubkeys((epoch_gauge_voter.key(), __pda_address)),
            );
        }
        {
            let my_key = epoch_gauge_voter.gauge_voter;
            let target_key = gauge_voter.key();
            if my_key != target_key {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintHasOne,
                        )
                        .with_account_name("epoch_gauge_voter")
                        .with_pubkeys((my_key, target_key)),
                );
            }
        }
        {
            let my_owner = AsRef::<AccountInfo>::as_ref(&epoch_gauge_voter).owner;
            let owner_address = gauge_program.key();
            if my_owner != &owner_address {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintOwner,
                        )
                        .with_account_name("epoch_gauge_voter")
                        .with_pubkeys((*my_owner, owner_address)),
                );
            }
        }
        {
            let my_key = gauge.gaugemeister;
            let target_key = gaugemeister.key();
            if my_key != target_key {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintHasOne,
                        )
                        .with_account_name("gauge")
                        .with_pubkeys((my_key, target_key)),
                );
            }
        }
        if !(!gauge.is_disabled) {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRaw,
                    )
                    .with_account_name("gauge"),
            );
        }
        {
            let my_key = epoch_gauge.gauge;
            let target_key = gauge.key();
            if my_key != target_key {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintHasOne,
                        )
                        .with_account_name("epoch_gauge")
                        .with_pubkeys((my_key, target_key)),
                );
            }
        }
        {
            let my_owner = AsRef::<AccountInfo>::as_ref(&epoch_gauge).owner;
            let owner_address = gauge_program.key();
            if my_owner != &owner_address {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintOwner,
                        )
                        .with_account_name("epoch_gauge")
                        .with_pubkeys((*my_owner, owner_address)),
                );
            }
        }
        if !AsRef::<AccountInfo>::as_ref(&epoch_gauge_vote).is_writable {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("epoch_gauge_vote"),
            );
        }
        {
            let my_owner = AsRef::<AccountInfo>::as_ref(&epoch_gauge_vote).owner;
            let owner_address = gauge_program.key();
            if my_owner != &owner_address {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintOwner,
                        )
                        .with_account_name("epoch_gauge_vote")
                        .with_pubkeys((*my_owner, owner_address)),
                );
            }
        }
        Ok(ClaimVotePayment {
            script_authority,
            seller,
            seller_token_account,
            token_vault,
            mint,
            config,
            vote_buy,
            vote_delegate,
            escrow,
            gaugemeister,
            gauge_voter,
            gauge_vote,
            epoch_gauge_voter,
            gauge,
            epoch_gauge,
            epoch_gauge_vote,
            gauge_program,
            locked_voter_program,
            token_program,
            system_program,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for ClaimVotePayment<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.script_authority.to_account_infos());
        account_infos.extend(self.seller.to_account_infos());
        account_infos.extend(self.seller_token_account.to_account_infos());
        account_infos.extend(self.token_vault.to_account_infos());
        account_infos.extend(self.mint.to_account_infos());
        account_infos.extend(self.config.to_account_infos());
        account_infos.extend(self.vote_buy.to_account_infos());
        account_infos.extend(self.vote_delegate.to_account_infos());
        account_infos.extend(self.escrow.to_account_infos());
        account_infos.extend(self.gaugemeister.to_account_infos());
        account_infos.extend(self.gauge_voter.to_account_infos());
        account_infos.extend(self.gauge_vote.to_account_infos());
        account_infos.extend(self.epoch_gauge_voter.to_account_infos());
        account_infos.extend(self.gauge.to_account_infos());
        account_infos.extend(self.epoch_gauge.to_account_infos());
        account_infos.extend(self.epoch_gauge_vote.to_account_infos());
        account_infos.extend(self.gauge_program.to_account_infos());
        account_infos.extend(self.locked_voter_program.to_account_infos());
        account_infos.extend(self.token_program.to_account_infos());
        account_infos.extend(self.system_program.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for ClaimVotePayment<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.script_authority.to_account_metas(None));
        account_metas.extend(self.seller.to_account_metas(None));
        account_metas.extend(self.seller_token_account.to_account_metas(None));
        account_metas.extend(self.token_vault.to_account_metas(None));
        account_metas.extend(self.mint.to_account_metas(None));
        account_metas.extend(self.config.to_account_metas(None));
        account_metas.extend(self.vote_buy.to_account_metas(None));
        account_metas.extend(self.vote_delegate.to_account_metas(None));
        account_metas.extend(self.escrow.to_account_metas(None));
        account_metas.extend(self.gaugemeister.to_account_metas(None));
        account_metas.extend(self.gauge_voter.to_account_metas(None));
        account_metas.extend(self.gauge_vote.to_account_metas(None));
        account_metas.extend(self.epoch_gauge_voter.to_account_metas(None));
        account_metas.extend(self.gauge.to_account_metas(None));
        account_metas.extend(self.epoch_gauge.to_account_metas(None));
        account_metas.extend(self.epoch_gauge_vote.to_account_metas(None));
        account_metas.extend(self.gauge_program.to_account_metas(None));
        account_metas.extend(self.locked_voter_program.to_account_metas(None));
        account_metas.extend(self.token_program.to_account_metas(None));
        account_metas.extend(self.system_program.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for ClaimVotePayment<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::Result<()> {
        anchor_lang::AccountsExit::exit(&self.seller, program_id)
            .map_err(|e| e.with_account_name("seller"))?;
        anchor_lang::AccountsExit::exit(&self.seller_token_account, program_id)
            .map_err(|e| e.with_account_name("seller_token_account"))?;
        anchor_lang::AccountsExit::exit(&self.token_vault, program_id)
            .map_err(|e| e.with_account_name("token_vault"))?;
        anchor_lang::AccountsExit::exit(&self.vote_buy, program_id)
            .map_err(|e| e.with_account_name("vote_buy"))?;
        anchor_lang::AccountsExit::exit(&self.vote_delegate, program_id)
            .map_err(|e| e.with_account_name("vote_delegate"))?;
        anchor_lang::AccountsExit::exit(&self.epoch_gauge_vote, program_id)
            .map_err(|e| e.with_account_name("epoch_gauge_vote"))?;
        Ok(())
    }
}
pub struct ClaimVotePaymentBumps {
    pub vote_buy: u8,
    pub vote_delegate: u8,
    pub escrow: u8,
    pub gauge_voter: u8,
    pub gauge_vote: u8,
    pub epoch_gauge_voter: u8,
}
#[automatically_derived]
impl ::core::fmt::Debug for ClaimVotePaymentBumps {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let names: &'static _ = &[
            "vote_buy",
            "vote_delegate",
            "escrow",
            "gauge_voter",
            "gauge_vote",
            "epoch_gauge_voter",
        ];
        let values: &[&dyn ::core::fmt::Debug] = &[
            &self.vote_buy,
            &self.vote_delegate,
            &self.escrow,
            &self.gauge_voter,
            &self.gauge_vote,
            &&self.epoch_gauge_voter,
        ];
        ::core::fmt::Formatter::debug_struct_fields_finish(
            f,
            "ClaimVotePaymentBumps",
            names,
            values,
        )
    }
}
impl Default for ClaimVotePaymentBumps {
    fn default() -> Self {
        ClaimVotePaymentBumps {
            vote_buy: u8::MAX,
            vote_delegate: u8::MAX,
            escrow: u8::MAX,
            gauge_voter: u8::MAX,
            gauge_vote: u8::MAX,
            epoch_gauge_voter: u8::MAX,
        }
    }
}
impl<'info> anchor_lang::Bumps for ClaimVotePayment<'info>
where
    'info: 'info,
{
    type Bumps = ClaimVotePaymentBumps;
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_claim_vote_payment {
    use super::*;
    use anchor_lang::prelude::borsh;
    /// Generated client accounts for [`ClaimVotePayment`].
    pub struct ClaimVotePayment {
        pub script_authority: Pubkey,
        pub seller: Pubkey,
        pub seller_token_account: Pubkey,
        pub token_vault: Pubkey,
        pub mint: Pubkey,
        pub config: Pubkey,
        pub vote_buy: Pubkey,
        pub vote_delegate: Pubkey,
        pub escrow: Pubkey,
        pub gaugemeister: Pubkey,
        pub gauge_voter: Pubkey,
        pub gauge_vote: Pubkey,
        pub epoch_gauge_voter: Pubkey,
        pub gauge: Pubkey,
        pub epoch_gauge: Pubkey,
        pub epoch_gauge_vote: Pubkey,
        pub gauge_program: Pubkey,
        pub locked_voter_program: Pubkey,
        pub token_program: Pubkey,
        pub system_program: Pubkey,
    }
    impl borsh::ser::BorshSerialize for ClaimVotePayment
    where
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.script_authority, writer)?;
            borsh::BorshSerialize::serialize(&self.seller, writer)?;
            borsh::BorshSerialize::serialize(&self.seller_token_account, writer)?;
            borsh::BorshSerialize::serialize(&self.token_vault, writer)?;
            borsh::BorshSerialize::serialize(&self.mint, writer)?;
            borsh::BorshSerialize::serialize(&self.config, writer)?;
            borsh::BorshSerialize::serialize(&self.vote_buy, writer)?;
            borsh::BorshSerialize::serialize(&self.vote_delegate, writer)?;
            borsh::BorshSerialize::serialize(&self.escrow, writer)?;
            borsh::BorshSerialize::serialize(&self.gaugemeister, writer)?;
            borsh::BorshSerialize::serialize(&self.gauge_voter, writer)?;
            borsh::BorshSerialize::serialize(&self.gauge_vote, writer)?;
            borsh::BorshSerialize::serialize(&self.epoch_gauge_voter, writer)?;
            borsh::BorshSerialize::serialize(&self.gauge, writer)?;
            borsh::BorshSerialize::serialize(&self.epoch_gauge, writer)?;
            borsh::BorshSerialize::serialize(&self.epoch_gauge_vote, writer)?;
            borsh::BorshSerialize::serialize(&self.gauge_program, writer)?;
            borsh::BorshSerialize::serialize(&self.locked_voter_program, writer)?;
            borsh::BorshSerialize::serialize(&self.token_program, writer)?;
            borsh::BorshSerialize::serialize(&self.system_program, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for ClaimVotePayment {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.script_authority,
                        true,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.seller,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.seller_token_account,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.token_vault,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.mint,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.config,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vote_buy,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vote_delegate,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.escrow,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.gaugemeister,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.gauge_voter,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.gauge_vote,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.epoch_gauge_voter,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.gauge,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.epoch_gauge,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.epoch_gauge_vote,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.gauge_program,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.locked_voter_program,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.token_program,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.system_program,
                        false,
                    ),
                );
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// [`cpi::accounts`] module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_claim_vote_payment {
    use super::*;
    /// Generated CPI struct of the accounts for [`ClaimVotePayment`].
    pub struct ClaimVotePayment<'info> {
        pub script_authority: anchor_lang::solana_program::account_info::AccountInfo<
            'info,
        >,
        pub seller: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub seller_token_account: anchor_lang::solana_program::account_info::AccountInfo<
            'info,
        >,
        pub token_vault: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub mint: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub config: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub vote_buy: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub vote_delegate: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub escrow: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub gaugemeister: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub gauge_voter: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub gauge_vote: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub epoch_gauge_voter: anchor_lang::solana_program::account_info::AccountInfo<
            'info,
        >,
        pub gauge: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub epoch_gauge: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub epoch_gauge_vote: anchor_lang::solana_program::account_info::AccountInfo<
            'info,
        >,
        pub gauge_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub locked_voter_program: anchor_lang::solana_program::account_info::AccountInfo<
            'info,
        >,
        pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub system_program: anchor_lang::solana_program::account_info::AccountInfo<
            'info,
        >,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for ClaimVotePayment<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.script_authority),
                        true,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.seller),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.seller_token_account),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.token_vault),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.mint),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.config),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vote_buy),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vote_delegate),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.escrow),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.gaugemeister),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.gauge_voter),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.gauge_vote),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.epoch_gauge_voter),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.gauge),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.epoch_gauge),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.epoch_gauge_vote),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.gauge_program),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.locked_voter_program),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.token_program),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.system_program),
                        false,
                    ),
                );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for ClaimVotePayment<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos
                .extend(
                    anchor_lang::ToAccountInfos::to_account_infos(&self.script_authority),
                );
            account_infos
                .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.seller));
            account_infos
                .extend(
                    anchor_lang::ToAccountInfos::to_account_infos(
                        &self.seller_token_account,
                    ),
                );
            account_infos
                .extend(
                    anchor_lang::ToAccountInfos::to_account_infos(&self.token_vault),
                );
            account_infos
                .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.mint));
            account_infos
                .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.config));
            account_infos
                .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.vote_buy));
            account_infos
                .extend(
                    anchor_lang::ToAccountInfos::to_account_infos(&self.vote_delegate),
                );
            account_infos
                .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.escrow));
            account_infos
                .extend(
                    anchor_lang::ToAccountInfos::to_account_infos(&self.gaugemeister),
                );
            account_infos
                .extend(
                    anchor_lang::ToAccountInfos::to_account_infos(&self.gauge_voter),
                );
            account_infos
                .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.gauge_vote));
            account_infos
                .extend(
                    anchor_lang::ToAccountInfos::to_account_infos(
                        &self.epoch_gauge_voter,
                    ),
                );
            account_infos
                .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.gauge));
            account_infos
                .extend(
                    anchor_lang::ToAccountInfos::to_account_infos(&self.epoch_gauge),
                );
            account_infos
                .extend(
                    anchor_lang::ToAccountInfos::to_account_infos(&self.epoch_gauge_vote),
                );
            account_infos
                .extend(
                    anchor_lang::ToAccountInfos::to_account_infos(&self.gauge_program),
                );
            account_infos
                .extend(
                    anchor_lang::ToAccountInfos::to_account_infos(
                        &self.locked_voter_program,
                    ),
                );
            account_infos
                .extend(
                    anchor_lang::ToAccountInfos::to_account_infos(&self.token_program),
                );
            account_infos
                .extend(
                    anchor_lang::ToAccountInfos::to_account_infos(&self.system_program),
                );
            account_infos
        }
    }
}
#[instruction(weight:u32)]
pub struct Vote<'info> {
    #[account(has_one = gaugemeister, has_one = script_authority)]
    pub config: Account<'info, VoteMarketConfig>,
    pub script_authority: Signer<'info>,
    #[account(owner = gauge_program.key())]
    pub gaugemeister: Account<'info, gauge_state::Gaugemeister>,
    #[account(has_one = gaugemeister, constraint = !gauge.is_disabled)]
    pub gauge: Account<'info, gauge_state::Gauge>,
    #[account(
        mut,
        seeds = [b"GaugeVoter",
        gaugemeister.key().as_ref(),
        escrow.key().as_ref()],
        bump,
        seeds::program = gauge_program.key(),
    )]
    pub gauge_voter: Account<'info, gauge_state::GaugeVoter>,
    #[account(
        mut,
        seeds = [b"GaugeVote",
        gauge_voter.key().as_ref(),
        gauge.key().as_ref()],
        bump,
        seeds::program = gauge_program.key(),
    )]
    pub gauge_vote: Account<'info, gauge_state::GaugeVote>,
    #[account(
        has_one = vote_delegate,
        seeds = [b"Escrow",
        gaugemeister.locker.as_ref(),
        escrow.owner.as_ref()],
        bump,
        seeds::program = locked_voter_state::id()
    )]
    pub escrow: Account<'info, locked_voter_state::Escrow>,
    #[account(mut, seeds = [b"vote-delegate", config.key().as_ref()], bump)]
    pub vote_delegate: SystemAccount<'info>,
    pub gauge_program: Program<'info, GaugeProgram>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info, VoteBumps> for Vote<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
            'info,
        >],
        __ix_data: &[u8],
        __bumps: &mut VoteBumps,
        __reallocs: &mut std::collections::BTreeSet<
            anchor_lang::solana_program::pubkey::Pubkey,
        >,
    ) -> anchor_lang::Result<Self> {
        let mut __ix_data = __ix_data;
        struct __Args {
            weight: u32,
        }
        impl borsh::ser::BorshSerialize for __Args
        where
            u32: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.weight, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for __Args
        where
            u32: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    weight: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        let __Args { weight } = __Args::deserialize(&mut __ix_data)
            .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
        let config: anchor_lang::accounts::account::Account<VoteMarketConfig> = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("config"))?;
        let script_authority: Signer = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("script_authority"))?;
        let gaugemeister: anchor_lang::accounts::account::Account<
            gauge_state::Gaugemeister,
        > = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("gaugemeister"))?;
        let gauge: anchor_lang::accounts::account::Account<gauge_state::Gauge> = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("gauge"))?;
        let gauge_voter: anchor_lang::accounts::account::Account<
            gauge_state::GaugeVoter,
        > = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("gauge_voter"))?;
        let gauge_vote: anchor_lang::accounts::account::Account<
            gauge_state::GaugeVote,
        > = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("gauge_vote"))?;
        let escrow: anchor_lang::accounts::account::Account<
            locked_voter_state::Escrow,
        > = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("escrow"))?;
        let vote_delegate: SystemAccount = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("vote_delegate"))?;
        let gauge_program: anchor_lang::accounts::program::Program<GaugeProgram> = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("gauge_program"))?;
        {
            let my_key = config.gaugemeister;
            let target_key = gaugemeister.key();
            if my_key != target_key {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintHasOne,
                        )
                        .with_account_name("config")
                        .with_pubkeys((my_key, target_key)),
                );
            }
        }
        {
            let my_key = config.script_authority;
            let target_key = script_authority.key();
            if my_key != target_key {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintHasOne,
                        )
                        .with_account_name("config")
                        .with_pubkeys((my_key, target_key)),
                );
            }
        }
        {
            let my_owner = AsRef::<AccountInfo>::as_ref(&gaugemeister).owner;
            let owner_address = gauge_program.key();
            if my_owner != &owner_address {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintOwner,
                        )
                        .with_account_name("gaugemeister")
                        .with_pubkeys((*my_owner, owner_address)),
                );
            }
        }
        {
            let my_key = gauge.gaugemeister;
            let target_key = gaugemeister.key();
            if my_key != target_key {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintHasOne,
                        )
                        .with_account_name("gauge")
                        .with_pubkeys((my_key, target_key)),
                );
            }
        }
        if !(!gauge.is_disabled) {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRaw,
                    )
                    .with_account_name("gauge"),
            );
        }
        let (__pda_address, __bump) = Pubkey::find_program_address(
            &[b"GaugeVoter", gaugemeister.key().as_ref(), escrow.key().as_ref()],
            &gauge_program.key().key(),
        );
        __bumps.gauge_voter = __bump;
        if gauge_voter.key() != __pda_address {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("gauge_voter")
                    .with_pubkeys((gauge_voter.key(), __pda_address)),
            );
        }
        if !AsRef::<AccountInfo>::as_ref(&gauge_voter).is_writable {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("gauge_voter"),
            );
        }
        let (__pda_address, __bump) = Pubkey::find_program_address(
            &[b"GaugeVote", gauge_voter.key().as_ref(), gauge.key().as_ref()],
            &gauge_program.key().key(),
        );
        __bumps.gauge_vote = __bump;
        if gauge_vote.key() != __pda_address {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("gauge_vote")
                    .with_pubkeys((gauge_vote.key(), __pda_address)),
            );
        }
        if !AsRef::<AccountInfo>::as_ref(&gauge_vote).is_writable {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("gauge_vote"),
            );
        }
        let (__pda_address, __bump) = Pubkey::find_program_address(
            &[b"Escrow", gaugemeister.locker.as_ref(), escrow.owner.as_ref()],
            &locked_voter_state::id().key(),
        );
        __bumps.escrow = __bump;
        if escrow.key() != __pda_address {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("escrow")
                    .with_pubkeys((escrow.key(), __pda_address)),
            );
        }
        {
            let my_key = escrow.vote_delegate;
            let target_key = vote_delegate.key();
            if my_key != target_key {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintHasOne,
                        )
                        .with_account_name("escrow")
                        .with_pubkeys((my_key, target_key)),
                );
            }
        }
        let (__pda_address, __bump) = Pubkey::find_program_address(
            &[b"vote-delegate", config.key().as_ref()],
            &__program_id,
        );
        __bumps.vote_delegate = __bump;
        if vote_delegate.key() != __pda_address {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("vote_delegate")
                    .with_pubkeys((vote_delegate.key(), __pda_address)),
            );
        }
        if !AsRef::<AccountInfo>::as_ref(&vote_delegate).is_writable {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vote_delegate"),
            );
        }
        Ok(Vote {
            config,
            script_authority,
            gaugemeister,
            gauge,
            gauge_voter,
            gauge_vote,
            escrow,
            vote_delegate,
            gauge_program,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for Vote<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.config.to_account_infos());
        account_infos.extend(self.script_authority.to_account_infos());
        account_infos.extend(self.gaugemeister.to_account_infos());
        account_infos.extend(self.gauge.to_account_infos());
        account_infos.extend(self.gauge_voter.to_account_infos());
        account_infos.extend(self.gauge_vote.to_account_infos());
        account_infos.extend(self.escrow.to_account_infos());
        account_infos.extend(self.vote_delegate.to_account_infos());
        account_infos.extend(self.gauge_program.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for Vote<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.config.to_account_metas(None));
        account_metas.extend(self.script_authority.to_account_metas(None));
        account_metas.extend(self.gaugemeister.to_account_metas(None));
        account_metas.extend(self.gauge.to_account_metas(None));
        account_metas.extend(self.gauge_voter.to_account_metas(None));
        account_metas.extend(self.gauge_vote.to_account_metas(None));
        account_metas.extend(self.escrow.to_account_metas(None));
        account_metas.extend(self.vote_delegate.to_account_metas(None));
        account_metas.extend(self.gauge_program.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for Vote<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::Result<()> {
        anchor_lang::AccountsExit::exit(&self.gauge_voter, program_id)
            .map_err(|e| e.with_account_name("gauge_voter"))?;
        anchor_lang::AccountsExit::exit(&self.gauge_vote, program_id)
            .map_err(|e| e.with_account_name("gauge_vote"))?;
        anchor_lang::AccountsExit::exit(&self.vote_delegate, program_id)
            .map_err(|e| e.with_account_name("vote_delegate"))?;
        Ok(())
    }
}
pub struct VoteBumps {
    pub gauge_voter: u8,
    pub gauge_vote: u8,
    pub escrow: u8,
    pub vote_delegate: u8,
}
#[automatically_derived]
impl ::core::fmt::Debug for VoteBumps {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f,
            "VoteBumps",
            "gauge_voter",
            &self.gauge_voter,
            "gauge_vote",
            &self.gauge_vote,
            "escrow",
            &self.escrow,
            "vote_delegate",
            &&self.vote_delegate,
        )
    }
}
impl Default for VoteBumps {
    fn default() -> Self {
        VoteBumps {
            gauge_voter: u8::MAX,
            gauge_vote: u8::MAX,
            escrow: u8::MAX,
            vote_delegate: u8::MAX,
        }
    }
}
impl<'info> anchor_lang::Bumps for Vote<'info>
where
    'info: 'info,
{
    type Bumps = VoteBumps;
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_vote {
    use super::*;
    use anchor_lang::prelude::borsh;
    /// Generated client accounts for [`Vote`].
    pub struct Vote {
        pub config: Pubkey,
        pub script_authority: Pubkey,
        pub gaugemeister: Pubkey,
        pub gauge: Pubkey,
        pub gauge_voter: Pubkey,
        pub gauge_vote: Pubkey,
        pub escrow: Pubkey,
        pub vote_delegate: Pubkey,
        pub gauge_program: Pubkey,
    }
    impl borsh::ser::BorshSerialize for Vote
    where
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.config, writer)?;
            borsh::BorshSerialize::serialize(&self.script_authority, writer)?;
            borsh::BorshSerialize::serialize(&self.gaugemeister, writer)?;
            borsh::BorshSerialize::serialize(&self.gauge, writer)?;
            borsh::BorshSerialize::serialize(&self.gauge_voter, writer)?;
            borsh::BorshSerialize::serialize(&self.gauge_vote, writer)?;
            borsh::BorshSerialize::serialize(&self.escrow, writer)?;
            borsh::BorshSerialize::serialize(&self.vote_delegate, writer)?;
            borsh::BorshSerialize::serialize(&self.gauge_program, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for Vote {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.config,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.script_authority,
                        true,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.gaugemeister,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.gauge,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.gauge_voter,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.gauge_vote,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.escrow,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vote_delegate,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.gauge_program,
                        false,
                    ),
                );
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// [`cpi::accounts`] module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_vote {
    use super::*;
    /// Generated CPI struct of the accounts for [`Vote`].
    pub struct Vote<'info> {
        pub config: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub script_authority: anchor_lang::solana_program::account_info::AccountInfo<
            'info,
        >,
        pub gaugemeister: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub gauge: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub gauge_voter: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub gauge_vote: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub escrow: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub vote_delegate: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub gauge_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for Vote<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.config),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.script_authority),
                        true,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.gaugemeister),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.gauge),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.gauge_voter),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.gauge_vote),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.escrow),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vote_delegate),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.gauge_program),
                        false,
                    ),
                );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for Vote<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos
                .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.config));
            account_infos
                .extend(
                    anchor_lang::ToAccountInfos::to_account_infos(&self.script_authority),
                );
            account_infos
                .extend(
                    anchor_lang::ToAccountInfos::to_account_infos(&self.gaugemeister),
                );
            account_infos
                .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.gauge));
            account_infos
                .extend(
                    anchor_lang::ToAccountInfos::to_account_infos(&self.gauge_voter),
                );
            account_infos
                .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.gauge_vote));
            account_infos
                .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.escrow));
            account_infos
                .extend(
                    anchor_lang::ToAccountInfos::to_account_infos(&self.vote_delegate),
                );
            account_infos
                .extend(
                    anchor_lang::ToAccountInfos::to_account_infos(&self.gauge_program),
                );
            account_infos
        }
    }
}
#[instruction(epoch:u32)]
pub struct SetMaxAmount<'info> {
    pub config: Account<'info, VoteMarketConfig>,
    #[account(
        mut,
        seeds = [b"vote-buy".as_ref(),
        epoch.to_le_bytes().as_ref(),
        config.key().as_ref(),
        gauge.key().as_ref()],
        bump
    )]
    pub vote_buy: Account<'info, VoteBuy>,
    #[account(
        constraint = config.gaugemeister = = gauge.gaugemeister,
        constraint = !gauge.is_disabled
    )]
    pub gauge: Account<'info, gauge_state::Gauge>,
    #[account(address = config.script_authority)]
    pub script_authority: Signer<'info>,
}
#[automatically_derived]
impl<'info> anchor_lang::Accounts<'info, SetMaxAmountBumps> for SetMaxAmount<'info>
where
    'info: 'info,
{
    #[inline(never)]
    fn try_accounts(
        __program_id: &anchor_lang::solana_program::pubkey::Pubkey,
        __accounts: &mut &'info [anchor_lang::solana_program::account_info::AccountInfo<
            'info,
        >],
        __ix_data: &[u8],
        __bumps: &mut SetMaxAmountBumps,
        __reallocs: &mut std::collections::BTreeSet<
            anchor_lang::solana_program::pubkey::Pubkey,
        >,
    ) -> anchor_lang::Result<Self> {
        let mut __ix_data = __ix_data;
        struct __Args {
            epoch: u32,
        }
        impl borsh::ser::BorshSerialize for __Args
        where
            u32: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.epoch, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for __Args
        where
            u32: borsh::BorshDeserialize,
        {
            fn deserialize_reader<R: borsh::maybestd::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    epoch: borsh::BorshDeserialize::deserialize_reader(reader)?,
                })
            }
        }
        let __Args { epoch } = __Args::deserialize(&mut __ix_data)
            .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
        let config: anchor_lang::accounts::account::Account<VoteMarketConfig> = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("config"))?;
        let vote_buy: anchor_lang::accounts::account::Account<VoteBuy> = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("vote_buy"))?;
        let gauge: anchor_lang::accounts::account::Account<gauge_state::Gauge> = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("gauge"))?;
        let script_authority: Signer = anchor_lang::Accounts::try_accounts(
                __program_id,
                __accounts,
                __ix_data,
                __bumps,
                __reallocs,
            )
            .map_err(|e| e.with_account_name("script_authority"))?;
        let (__pda_address, __bump) = Pubkey::find_program_address(
            &[
                b"vote-buy".as_ref(),
                epoch.to_le_bytes().as_ref(),
                config.key().as_ref(),
                gauge.key().as_ref(),
            ],
            &__program_id,
        );
        __bumps.vote_buy = __bump;
        if vote_buy.key() != __pda_address {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("vote_buy")
                    .with_pubkeys((vote_buy.key(), __pda_address)),
            );
        }
        if !AsRef::<AccountInfo>::as_ref(&vote_buy).is_writable {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vote_buy"),
            );
        }
        if !(config.gaugemeister == gauge.gaugemeister) {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRaw,
                    )
                    .with_account_name("gauge"),
            );
        }
        if !(!gauge.is_disabled) {
            return Err(
                anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRaw,
                    )
                    .with_account_name("gauge"),
            );
        }
        {
            let actual = script_authority.key();
            let expected = config.script_authority;
            if actual != expected {
                return Err(
                    anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintAddress,
                        )
                        .with_account_name("script_authority")
                        .with_pubkeys((actual, expected)),
                );
            }
        }
        Ok(SetMaxAmount {
            config,
            vote_buy,
            gauge,
            script_authority,
        })
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountInfos<'info> for SetMaxAmount<'info>
where
    'info: 'info,
{
    fn to_account_infos(
        &self,
    ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
        let mut account_infos = ::alloc::vec::Vec::new();
        account_infos.extend(self.config.to_account_infos());
        account_infos.extend(self.vote_buy.to_account_infos());
        account_infos.extend(self.gauge.to_account_infos());
        account_infos.extend(self.script_authority.to_account_infos());
        account_infos
    }
}
#[automatically_derived]
impl<'info> anchor_lang::ToAccountMetas for SetMaxAmount<'info> {
    fn to_account_metas(
        &self,
        is_signer: Option<bool>,
    ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
        let mut account_metas = ::alloc::vec::Vec::new();
        account_metas.extend(self.config.to_account_metas(None));
        account_metas.extend(self.vote_buy.to_account_metas(None));
        account_metas.extend(self.gauge.to_account_metas(None));
        account_metas.extend(self.script_authority.to_account_metas(None));
        account_metas
    }
}
#[automatically_derived]
impl<'info> anchor_lang::AccountsExit<'info> for SetMaxAmount<'info>
where
    'info: 'info,
{
    fn exit(
        &self,
        program_id: &anchor_lang::solana_program::pubkey::Pubkey,
    ) -> anchor_lang::Result<()> {
        anchor_lang::AccountsExit::exit(&self.vote_buy, program_id)
            .map_err(|e| e.with_account_name("vote_buy"))?;
        Ok(())
    }
}
pub struct SetMaxAmountBumps {
    pub vote_buy: u8,
}
#[automatically_derived]
impl ::core::fmt::Debug for SetMaxAmountBumps {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "SetMaxAmountBumps",
            "vote_buy",
            &&self.vote_buy,
        )
    }
}
impl Default for SetMaxAmountBumps {
    fn default() -> Self {
        SetMaxAmountBumps {
            vote_buy: u8::MAX,
        }
    }
}
impl<'info> anchor_lang::Bumps for SetMaxAmount<'info>
where
    'info: 'info,
{
    type Bumps = SetMaxAmountBumps;
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a struct for a given
/// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
/// instead of an `AccountInfo`. This is useful for clients that want
/// to generate a list of accounts, without explicitly knowing the
/// order all the fields should be in.
///
/// To access the struct in this module, one should use the sibling
/// `accounts` module (also generated), which re-exports this.
pub(crate) mod __client_accounts_set_max_amount {
    use super::*;
    use anchor_lang::prelude::borsh;
    /// Generated client accounts for [`SetMaxAmount`].
    pub struct SetMaxAmount {
        pub config: Pubkey,
        pub vote_buy: Pubkey,
        pub gauge: Pubkey,
        pub script_authority: Pubkey,
    }
    impl borsh::ser::BorshSerialize for SetMaxAmount
    where
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.config, writer)?;
            borsh::BorshSerialize::serialize(&self.vote_buy, writer)?;
            borsh::BorshSerialize::serialize(&self.gauge, writer)?;
            borsh::BorshSerialize::serialize(&self.script_authority, writer)?;
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::ToAccountMetas for SetMaxAmount {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.config,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vote_buy,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.gauge,
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        self.script_authority,
                        true,
                    ),
                );
            account_metas
        }
    }
}
/// An internal, Anchor generated module. This is used (as an
/// implementation detail), to generate a CPI struct for a given
/// `#[derive(Accounts)]` implementation, where each field is an
/// AccountInfo.
///
/// To access the struct in this module, one should use the sibling
/// [`cpi::accounts`] module (also generated), which re-exports this.
pub(crate) mod __cpi_client_accounts_set_max_amount {
    use super::*;
    /// Generated CPI struct of the accounts for [`SetMaxAmount`].
    pub struct SetMaxAmount<'info> {
        pub config: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub vote_buy: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub gauge: anchor_lang::solana_program::account_info::AccountInfo<'info>,
        pub script_authority: anchor_lang::solana_program::account_info::AccountInfo<
            'info,
        >,
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountMetas for SetMaxAmount<'info> {
        fn to_account_metas(
            &self,
            is_signer: Option<bool>,
        ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
            let mut account_metas = ::alloc::vec::Vec::new();
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.config),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vote_buy),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.gauge),
                        false,
                    ),
                );
            account_metas
                .push(
                    anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                        anchor_lang::Key::key(&self.script_authority),
                        true,
                    ),
                );
            account_metas
        }
    }
    #[automatically_derived]
    impl<'info> anchor_lang::ToAccountInfos<'info> for SetMaxAmount<'info> {
        fn to_account_infos(
            &self,
        ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
            let mut account_infos = ::alloc::vec::Vec::new();
            account_infos
                .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.config));
            account_infos
                .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.vote_buy));
            account_infos
                .extend(anchor_lang::ToAccountInfos::to_account_infos(&self.gauge));
            account_infos
                .extend(
                    anchor_lang::ToAccountInfos::to_account_infos(&self.script_authority),
                );
            account_infos
        }
    }
}
