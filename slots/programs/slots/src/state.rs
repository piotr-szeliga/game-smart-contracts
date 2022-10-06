use anchor_lang::prelude::*;

#[account]
pub struct Game {
    pub authority: Pubkey,
    pub name: String,
    pub bump: u8,
    pub token_type: bool,
    pub royalty: u8,
    pub community_wallet: Pubkey,
    pub main_balance: u64,
    pub community_balance: u64,
    pub jackpot: u64,
}
impl Game
{
    pub const LEN: usize =  std::mem::size_of::<Game>();
}

#[account]
pub struct Player {
    pub game: Pubkey,
    pub earned_money: u64,
    pub key: Pubkey,
    pub status: u32,
    pub bump: u8,
}
impl Player
{
    pub const LEN: usize =  std::mem::size_of::<Player>();
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized wallet cannot create game")]
    UnauthorizedWallet,
}