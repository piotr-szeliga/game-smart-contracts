use anchor_lang::prelude::*;

#[account]
pub struct NftVault {
    pub authority: Pubkey,
    pub pool_bump: u8,
    pub mint_price: u64,
    pub total_supply: u32,
    pub sold_mints: Vec<Pubkey>,
}


#[error_code]
pub enum ErrorCode
{
    #[msg("Not Enough Tokens.")] // 
    NotEnoughTokens,
    #[msg("Already Minted")] //
    AlreadyMinted,
    #[msg("Not Enough SOL")] //
    NotEnoughSol
}