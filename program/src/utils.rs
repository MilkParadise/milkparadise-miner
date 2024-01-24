use crate::{constants::*, error::*, instructions::*, states::*};
use anchor_lang::prelude::*;

pub fn calculate_trade(global_state: &GlobalState, rt: u128, rs: u128, bs: u128) -> Result<u64> {
    let psn = global_state.psn as u128;
    let psnh = global_state.psnh as u128;
    let divee = psn
        .checked_mul(rs)
        .unwrap()
        .checked_add(psnh.checked_mul(rt).unwrap())
        .unwrap()
        .checked_div(rt)
        .unwrap()
        .checked_add(psnh)
        .unwrap();
    msg!("calculate_trade x {}", psn.checked_mul(bs).unwrap());
    msg!("calculate_trade divee {}", divee);
    let res = psn.checked_mul(bs).unwrap().checked_div(divee).unwrap();
    Ok(res as u64)
}

pub fn calculate_items_buy(global_state: &GlobalState, my_sol: u64, total_sol: u64) -> Result<u64> {
    calculate_trade(
        global_state,
        my_sol as u128,
        total_sol as u128,
        global_state.market_items as u128,
    )
}

pub fn calculate_items_sell(
    global_state: &GlobalState,
    my_items: u64,
    total_sol_amt: u64,
) -> Result<u64> {
    msg!("my items {}", my_items);
    msg!("global_state.market_items {}", global_state.market_items);
    msg!("global_state.total_sol_amt {}", total_sol_amt);
    calculate_trade(
        global_state,
        my_items as u128,
        global_state.market_items as u128,
        total_sol_amt as u128,
    )
}

pub fn dev_fee(global_state: &GlobalState, amount: u64) -> Result<u64> {
    let res = (amount as u128) * (global_state.dev_fee as u128) / 10000;
    Ok(res as u64)
}

pub fn get_items_since_last_harvest(
    user_state: &UserState,
    cur_timestamp: u64,
    items_per_miner: u64,
) -> Result<u64> {
    let mut seconds_passed = cur_timestamp
        .checked_sub(user_state.last_harvest_time)
        .unwrap();
    if seconds_passed > items_per_miner {
        seconds_passed = items_per_miner;
    }
    msg!("seconds passed {}", seconds_passed);
    msg!("user_state.miners {}", user_state.miners);
    Ok(seconds_passed * user_state.miners)
}
