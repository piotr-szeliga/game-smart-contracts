use anchor_lang::prelude::*;
use anchor_spl::token::{Token};

use crate::state::*;
use crate::constants::*;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateGame<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,
  
  #[account(
    init,
    payer = payer,
    space = Game::LEN + 8,
    seeds = [
      name.as_bytes(),
      GAME_SEED_PREFIX.as_bytes(),
      payer.key().as_ref()
    ],
    bump
  )]
  pub game: Account<'info, Game>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FundPrize<'info> {
  #[account(mut)]
  pub funder: Signer<'info>,

  #[account(
    mut,
    seeds = [
      game.name.as_bytes(),
      GAME_SEED_PREFIX.as_bytes(),
      game.authority.as_ref()
    ],
    bump = game.bump
  )]
  pub game: Account<'info, Game>,

  /// CHECK:
  #[account(mut)]
  pub game_ata: AccountInfo<'info>,

  /// CHECK:
  #[account(mut)]
  pub funder_ata: AccountInfo<'info>,

  /// CHECK:
  pub prize_mint: AccountInfo<'info>,

  pub token_program: Program<'info, Token>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawPrize<'info> {
  #[account(mut)]
  pub withdrawer: Signer<'info>,

  /// CHECK:
  #[account(mut)]
  pub withdrawer_ata: AccountInfo<'info>,

  #[account(
    mut,
    seeds = [
      game.name.as_bytes(),
      GAME_SEED_PREFIX.as_bytes(),
      game.authority.as_ref()
    ],
    bump = game.bump
  )]
  pub game: Account<'info, Game>,

  /// CHECK:
  #[account(mut)]
  pub game_ata: AccountInfo<'info>,

  pub token_program: Program<'info, Token>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ConfigGame<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,
  #[account(
    mut,
    seeds = [
      game.name.as_bytes(),
      GAME_SEED_PREFIX.as_bytes(),
      game.authority.as_ref(),
    ],
    bump = game.bump,
  )]
  pub game: Account<'info, Game>,
}

#[derive(Accounts)]
pub struct AddPlayer<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,
  #[account(
    seeds = [
      game.name.as_bytes(),
      GAME_SEED_PREFIX.as_bytes(),
      game.authority.as_ref(),
    ],
    bump = game.bump,
  )]
  pub game: Account<'info, Game>,
  #[account(
    init,
    payer = payer,
    space = Player::LEN + 8,
    seeds = [
      PLAYER_SEED_PREFIX.as_bytes(),
      payer.key().as_ref(),
      game.key().as_ref(),
    ],
    bump
  )]
  pub player: Account<'info, Player>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SendToCommunityWallet<'info> 
{
  #[account(
    mut,
    seeds = [
      game.name.as_bytes(),
      GAME_SEED_PREFIX.as_bytes(),
      game.authority.as_ref(),
    ],
    bump = game.bump,
  )]
  pub game: Account<'info, Game>,
  /// CHECK:
  #[account(mut)]
  pub game_treasury_ata: AccountInfo<'info>,
  #[account(
    mut,
    constraint = game.community_wallets.iter().any(|x| x == &community_wallet.key())
  )]
  pub community_wallet: SystemAccount<'info>,
  /// CHECK:
  #[account(mut)]
  pub community_treasury_ata: AccountInfo<'info>,
  pub token_program: Program<'info, Token>,
  pub system_program: Program<'info, System>,
}