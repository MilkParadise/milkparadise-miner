use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod states;
pub mod utils;

use instructions::*;

declare_id!("CoWBUXy3GYDjXsw285Pamfu7b8ybtUohBiVSMBh3j2uX");
#[program]
pub mod miner_app {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, new_authority: Pubkey) -> Result<()> {
        initialize::handle(ctx, new_authority)
    }

    pub fn buy_items(ctx: Context<BuyItems>, amount: u64) -> Result<()> {
        buy_items::handle(ctx, amount)
    }

    pub fn sell_items(ctx: Context<SellItems>) -> Result<()> {
        sell_items::handle(ctx)
    }

    pub fn harvest_items(ctx: Context<HarvestItems>) -> Result<()> {
        harvest_items::handle(ctx)
    }
}
