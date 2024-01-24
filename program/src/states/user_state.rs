use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct UserState {
    // to avoid reinitialization attack
    pub is_initialized: u8,
    pub user: Pubkey,
    pub last_harvest_time: u64,
    pub claimed_items: u64,
    pub miners: u64,
    pub referral: Pubkey,
    pub referral_set: u8,
    pub referrals_reward: u64,
}
