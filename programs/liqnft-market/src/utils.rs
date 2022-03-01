use anchor_lang::{
    prelude::*,
    solana_program::{self, instruction::Instruction, sysvar},
    Id,
};
use anchor_spl::associated_token::AssociatedToken;
use borsh::{BorshDeserialize, BorshSerialize};

/// creates a create_master_edition instruction
#[allow(clippy::too_many_arguments)]
pub fn create_auction_house(
    ah_program_id: Pubkey,
    treasury_mint: Pubkey,
    payer: Pubkey,
    authority: Pubkey,
    fee_withdrawal_destination: Pubkey,
    treasury_withdrawal_destination: Pubkey,
    treasury_withdrawal_destination_owner: Pubkey,
    auction_house: Pubkey,
    auction_house_fee_account: Pubkey,
    auction_house_treasury: Pubkey,
    bump: u8,
    fee_payer_bump: u8,
    treasury_bump: u8,
    seller_fee_basis_points: u16,
    requires_sign_off: bool,
    can_change_sale_price: bool,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new_readonly(treasury_mint, false),
        AccountMeta::new_readonly(payer, true),
        AccountMeta::new_readonly(authority, false),
        AccountMeta::new(fee_withdrawal_destination, false),
        AccountMeta::new(treasury_withdrawal_destination, false),
        AccountMeta::new_readonly(treasury_withdrawal_destination_owner, false),
        AccountMeta::new(auction_house, false),
        AccountMeta::new(auction_house_fee_account, false),
        AccountMeta::new(auction_house_treasury, false),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(solana_program::system_program::id(), false),
        AccountMeta::new_readonly(AssociatedToken::id(), false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
    ];

    Instruction {
        program_id: ah_program_id,
        accounts,
        data: AuctionHouseInstruction::CreateAuctionHouse(CreateAuctionHouseArgs {
            bump,
            fee_payer_bump,
            treasury_bump,
            seller_fee_basis_points,
            requires_sign_off,
            can_change_sale_price,
        })
        .try_to_vec()
        .unwrap(),
    }
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct CreateAuctionHouseArgs {
    bump: u8,
    fee_payer_bump: u8,
    treasury_bump: u8,
    seller_fee_basis_points: u16,
    requires_sign_off: bool,
    can_change_sale_price: bool,
}

/// Instructions supported by the Metadata program.
#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub enum AuctionHouseInstruction {
    CreateAuctionHouse(CreateAuctionHouseArgs),
}
