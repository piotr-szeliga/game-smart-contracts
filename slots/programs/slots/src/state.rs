use anchor_lang::prelude::*;

#[account]
pub struct Game {
    pub authority: Pubkey,
    pub name: String,
    pub bump: u8,
    pub treasury_bump: u8,
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
    pub treasury_bump: u8,
}
impl Player
{
    pub const LEN: usize =  std::mem::size_of::<Player>();
}