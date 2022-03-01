use anchor_lang::prelude::*;
use anchor_spl::{token::{Mint, Token}, associated_token::AssociatedToken};

#[derive(Accounts)]
pub struct Initialize {}


#[derive(Accounts)]
#[instruction(authority_bump: u8, fee_payer_bump: u8, treasury_bump: u8)]
pub struct CreateAuctionHouseWithProxy<'info> {
    pub treasury_mint: Account<'info, Mint>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub authority: UncheckedAccount<'info>,
    #[account(mut)]
    pub fee_withdrawal_destination: UncheckedAccount<'info>,
    #[account(mut)]
    pub treasury_withdrawal_destination: UncheckedAccount<'info>,
    pub treasury_withdrawal_destination_owner: UncheckedAccount<'info>,
    #[account(mut)]
    pub auction_house: UncheckedAccount<'info>,
    #[account(mut)]
    pub auction_house_fee_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub auction_house_treasury: UncheckedAccount<'info>,
    pub ah_program: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}