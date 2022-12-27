use anchor_lang::prelude::*;

#[account]
pub struct Gift 
{
    pub creator: Pubkey,

    pub spl_token_mint: Pubkey,

    pub destination_address: Pubkey,

    pub token_amount: u64,
    
    pub nft_mint: Pubkey,

    pub redeamed: bool,

    pub bump: u8,
}