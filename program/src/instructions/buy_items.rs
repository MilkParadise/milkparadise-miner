use crate::{constants::*, error::*, instructions::*, states::*, utils::*};
use anchor_lang::prelude::*;
use solana_program::{program::invoke, system_instruction};
use std::mem::size_of;
#[derive(Accounts)]
pub struct BuyItems<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
      mut,
      seeds = [GLOBAL_STATE_SEED],
      bump,
    )]
    pub global_state: Account<'info, GlobalState>,

    #[account(mut, address = global_state.treasury)]
    /// CHECK: this should be set by admin
    pub treasury: AccountInfo<'info>,

    #[account(
        mut,
        address = global_state.vault
    )]
    /// CHECK: this should be set by admin
    pub vault: AccountInfo<'info>,

    #[account(
        init_if_needed,
        seeds = [USER_STATE_SEED, user.key().as_ref()],
        bump,
        payer = user,
        space = 8 + size_of::<UserState>()
    )]
    pub user_state: Account<'info, UserState>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handle(ctx: Context<BuyItems>, sol_amount: u64) -> Result<()> {
    let accts = ctx.accounts;

    let cur_timestamp = Clock::get()?.unix_timestamp;
    if accts.user_state.is_initialized == 0 {
        accts.user_state.is_initialized = 1;
        accts.user_state.referrals_reward = 0 as u64;
        accts.user_state.last_harvest_time = cur_timestamp as u64;
        accts.user_state.user = accts.user.key();
    } else {
        require!(
            accts.user_state.user.eq(&accts.user.key()),
            AppError::IncorrectUserState
        );
    }

    let mut items_bought =
        calculate_items_buy(&accts.global_state, sol_amount, accts.vault.lamports())?;

    let items_bought_fee = dev_fee(&accts.global_state, items_bought)?;
    items_bought = items_bought.checked_sub(items_bought_fee).unwrap();

    let sol_fee = dev_fee(&accts.global_state, sol_amount)?;
    let real_sol = sol_amount.checked_sub(sol_fee).unwrap();

    // send sol_fee to treasury
    invoke(
        &system_instruction::transfer(&accts.user.key(), &accts.treasury.key(), sol_fee),
        &[
            accts.user.to_account_info().clone(),
            accts.treasury.clone(),
            accts.system_program.to_account_info().clone(),
        ],
    )?;
    // add vault <- sol_amount - fee
    invoke(
        &system_instruction::transfer(&accts.user.key(), &accts.vault.key(), real_sol),
        &[
            accts.user.to_account_info().clone(),
            accts.vault.clone(),
            accts.system_program.to_account_info().clone(),
        ],
    )?;
    accts.user_state.claimed_items = accts
        .user_state
        .claimed_items
        .checked_add(items_bought)
        .unwrap();
    Ok(())
}
