use anchor_lang::prelude::*;

#[account]
pub struct Global
{
    pub name: String,

    pub authority: Pubkey,

    pub expiration_period: u64,

    pub gate_token_mint: Pubkey,

    pub gate_token_amount: u64,

    pub bump: u8,
}

impl Global {
    pub const LEN: usize = std::mem::size_of::<Global>();
}

#[account]
pub struct Gift 
{
    pub creator: Pubkey,

    pub spl_token_mint: Pubkey,

    pub gate_token_mint: Pubkey,

    pub gate_token_amount: u64,

    pub destination_address: Pubkey,

    pub token_amount: u64,
    
    pub nft_mint: Pubkey,

    pub expiration_time: u64,

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
    #[msg("Time Expired")]
    Expired,
    #[msg("Invalid Gate Token Holder")]
    InvalidHolder,
}