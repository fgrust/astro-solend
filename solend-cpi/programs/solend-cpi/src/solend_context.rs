use crate::*;
use anchor_spl::token::Token;

#[derive(Accounts, Clone)]
pub struct InitializeObligation<'info> {
    #[account(mut)]
    pub obligation: AccountInfo<'info>,
    pub lending_market: AccountInfo<'info>,
    pub obligation_owner: Signer<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts, Clone)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub source_liquidity: AccountInfo<'info>,
    #[account(mut)]
    pub source_collateral: AccountInfo<'info>,
    #[account(mut)]
    pub reserve: AccountInfo<'info>,
    #[account(mut)]
    pub reserve_liquidity_supply: AccountInfo<'info>,
    #[account(mut)]
    pub reserve_collateral_mint: AccountInfo<'info>,
    pub lending_market: AccountInfo<'info>,
    pub lending_market_authority: AccountInfo<'info>,
    #[account(mut)]
    pub destination_collateral: AccountInfo<'info>,
    #[account(mut)]
    pub obligation: AccountInfo<'info>,
    pub obligation_owner: Signer<'info>,
    pub pyth_oracle: AccountInfo<'info>,
    pub switchboard_oracle: AccountInfo<'info>,
    pub user_transfer_authority: Signer<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts, Clone)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub source_collateral: AccountInfo<'info>,
    #[account(mut)]
    pub destination_collateral: AccountInfo<'info>,
    #[account(mut)]
    pub withdraw_reserve: AccountInfo<'info>,
    #[account(mut)]
    pub obligation: AccountInfo<'info>,
    pub lending_market: AccountInfo<'info>,
    pub lending_market_authority: AccountInfo<'info>,
    #[account(mut)]
    pub destination_liquidity: AccountInfo<'info>,
    #[account(mut)]
    pub reserve_collateral_mint: AccountInfo<'info>,
    #[account(mut)]
    pub reserve_liquidity_supply: AccountInfo<'info>,
    pub obligation_owner: Signer<'info>,
    pub user_transfer_authority: Signer<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts, Clone)]
pub struct Borrow<'info> {
    #[account(mut)]
    pub source_liquidity: AccountInfo<'info>,
    #[account(mut)]
    pub destination_liquidity: AccountInfo<'info>,
    #[account(mut)]
    pub borrow_reserve: AccountInfo<'info>,
    #[account(mut)]
    pub borrow_reserve_liquidity_fee_receiver: AccountInfo<'info>,
    #[account(mut)]
    pub obligation: AccountInfo<'info>,
    pub lending_market: AccountInfo<'info>,
    pub lending_market_authority: AccountInfo<'info>,
    pub obligation_owner: Signer<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts, Clone)]
pub struct Repay<'info> {
    #[account(mut)]
    pub source_liquidity: AccountInfo<'info>,
    #[account(mut)]
    pub destination_liquidity: AccountInfo<'info>,
    #[account(mut)]
    pub repay_reserve: AccountInfo<'info>,
    #[account(mut)]
    pub obligation: AccountInfo<'info>,
    pub lending_market: AccountInfo<'info>,
    pub user_transfer_authority: Signer<'info>,
    pub clock: Sysvar<'info, Clock>,
    pub token_program: Program<'info, Token>,
}
