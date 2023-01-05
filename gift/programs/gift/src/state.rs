use anchor_lang::prelude::*;

#[account]
pub struct Gift 
{
    pub creator: Pubkey,

    pub spl_token_mint: Pubkey,

    pub destination_address: Pubkey,

    pub token_amount: u64,
    
    pub nft_mint: Pubkey,

    pub redeemed: bool,

    pub bump: u8,
}

impl Gift {
    pub const LEN: usize = std::mem::size_of::<Gift>();
}

#[error_code]
pub enum ErrorCode
{
    #[msg("Mint Failed")]
    MintFailed,
    #[msg("Metadata Create Failed")]
    MetadataCreateFailed,
    #[msg("Already Redeemed")]
    AlreadyRedeemed,
}