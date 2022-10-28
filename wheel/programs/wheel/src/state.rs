use anchor_lang::prelude::*;
use crate::constants::*;

#[account]
pub struct Game {
  pub authority: Pubkey,
  pub name: String,
  pub bump: u8,
  pub cells: [Cell; MAX_CELL_COUNT],
}

impl Game {
  pub const LEN: usize = std::mem::size_of::<Game>();
}

#[derive(Debug, AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Copy)]
pub struct Cell {
  pub spl_mint: Pubkey,
  pub amount: u64,
  pub chance: u16,
}

impl Default for Cell {
  fn default() -> Self {
    Cell {
      spl_mint: Pubkey::default(),
      amount: 0,
      chance: 0,
    }
  }
}

#[account]
pub struct Player {
  pub game: Pubkey,
  pub key: Pubkey,
  pub bump: u8,
  pub status: u32,
}

impl Player {
  pub const LEN: usize = std::mem::size_of::<Player>();
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized wallet cannot create game")]
    UnauthorizedWallet,
    #[msg("There can be 12 cells on the wheel at most")]
    ExceedMaxCellCount
}