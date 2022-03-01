use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

use market_accounts::*;

mod constants;
mod utils;
mod market_accounts;

#[program]
pub mod liqnft_market {

    use anchor_lang::solana_program;

    use crate::utils::create_auction_house;


    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }


    pub fn create_auction_house_with_proxy(
        ctx: Context<CreateAuctionHouseWithProxy>,
        seller_fee_basis_points: u16,
        requires_sign_off: bool,
        can_change_sale_price: bool,
        authority_bump: u8,
        fee_payer_bump: u8,
        treasury_bump: u8,
    ) -> ProgramResult {
        let creator_ah_info = vec![
            // treasury mint
            ctx.accounts.treasury_mint.to_account_info(),
            // payer
            ctx.accounts.payer.to_account_info(),
            // authority
            ctx.accounts.authority.to_account_info(),
            // fee_withdrawal_destination
            ctx.accounts.fee_withdrawal_destination.to_account_info(),
            // treasury_withdrawal_destination
            ctx.accounts
                .treasury_withdrawal_destination
                .to_account_info(),
            // treasury_withdrawal_destination_owner
            ctx.accounts
                .treasury_withdrawal_destination_owner
                .to_account_info(),
            // auction_house
            ctx.accounts.auction_house.to_account_info(),
            // auction_house_fee_account
            ctx.accounts.auction_house_fee_account.to_account_info(),
            // auction_house_treasury
            ctx.accounts.auction_house_treasury.to_account_info(),
            // spl_token
            ctx.accounts.token_program.to_account_info(),
            // ata_program
            ctx.accounts.associated_token_program.to_account_info(),
            // system_program
            ctx.accounts.system_program.to_account_info(),
            // rent
            ctx.accounts.rent.to_account_info(),
        ];
        msg!("CPI");

        // payer will pay for authority
        solana_program::program::invoke(
            &create_auction_house(
                ctx.accounts.ah_program.key(),
                ctx.accounts.treasury_mint.key(),
                ctx.accounts.payer.key(),
                ctx.accounts.authority.key(),
                ctx.accounts.fee_withdrawal_destination.key(),
                ctx.accounts.treasury_withdrawal_destination.key(),
                ctx.accounts.treasury_withdrawal_destination_owner.key(),
                ctx.accounts.auction_house.key(),
                ctx.accounts.auction_house_fee_account.key(),
                ctx.accounts.auction_house_treasury.key(),
                authority_bump,
                fee_payer_bump,
                treasury_bump,
                seller_fee_basis_points,
                requires_sign_off,
                can_change_sale_price
            ),
            creator_ah_info.as_slice(),
        )?;

        Ok(())
    }
}

