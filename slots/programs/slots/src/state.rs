use anchor_lang::prelude::*;

#[account]
pub struct Game {
    pub authroity: Pubkey,
    pub bump: u8,
}
impl Game
{
    pub const LEN: usize =  std::mem::size_of::<Game>();
}

#[account]
pub struct Round {
    pub round_no: u32,
    pub player: Pubkey,
    pub status: [u8; 5],
    pub price: u64,
}
impl Round
{
    pub const LEN: usize =  std::mem::size_of::<Round>();
}