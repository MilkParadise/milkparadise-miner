use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct GlobalState {
    // to avoid reinitialization attack
    pub is_initialized: u8,
    pub authority: Pubkey,
    pub vault: Pubkey,
    pub treasury: Pubkey,
    pub market_items: u64,
    pub dev_fee: u64,
    pub psn: u64,
    pub psnh: u64,
    pub items_per_miner: u64,
}
