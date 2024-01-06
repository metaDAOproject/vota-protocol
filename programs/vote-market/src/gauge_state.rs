use anchor_lang::prelude::*;

/// Manages the rewards shares of all [Gauge]s of a [quarry_mine::rewarder].
#[account]
#[derive(Copy, Debug, Default)]
pub struct Gaugemeister {
    /// Base.
    pub base: Pubkey,
    /// Bump seed.
    pub bump: u8,

    /// The Rewarder.
    pub rewarder: Pubkey,
    /// The Quarry Operator.
    pub operator: Pubkey,
    /// The [locked_voter::Locker].
    pub locker: Pubkey,

    /// Account which may enable/disable gauges on the [Gaugemeister].
    /// May call the following instructions:
    /// - gauge_enable
    /// - gauge_disable
    pub foreman: Pubkey,
    /// Number of seconds per rewards epoch.
    /// This may be modified later.
    /// The epoch duration is not exact, as epochs must manually be incremented.
    pub epoch_duration_seconds: u32,

    /// The current rewards epoch.
    pub current_rewards_epoch: u32,
    /// When the next epoch starts.
    pub next_epoch_starts_at: u64,

    /// Token mint. Unused but useful for frontends.
    pub locker_token_mint: Pubkey,
    /// Governor associated with the Locker. Unused but useful for frontends.
    pub locker_governor: Pubkey,
}

/// A [Gauge] determines the rewards shares to give to a [quarry_mine::Quarry].
#[account]
#[derive(Copy, Debug, Default)]
pub struct Gauge {
    /// The [Gaugemeister].
    pub gaugemeister: Pubkey,
    /// The [quarry_mine::Quarry] being voted on.
    pub quarry: Pubkey,
    /// If true, this Gauge cannot receive any more votes
    /// and rewards shares cannot be synchronized from it.
    pub is_disabled: bool,
}