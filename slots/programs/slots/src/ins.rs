use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::state::*;
use crate::constants::*;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateGame<'info> 
{
    #[account(mut)]
    pub payer: Signer<'info>,
    // game
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
pub struct SetCommunityWallet<'info> {
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
}


#[derive(Accounts)]
pub struct SetJackpot<'info> {
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
pub struct Play<'info>
{
  #[account(mut)]
  pub payer: Signer<'info>,
  #[account(mut)]
  pub payer_ata: Account<'info, TokenAccount>,
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
  #[account(mut)]
  pub game_treasury_ata: Account<'info, TokenAccount>,
  #[account(
    mut,
    address = game.community_wallet
  )]
  pub community_treasury: SystemAccount<'info>,
  #[account(
    mut,
    constraint = community_treasury_ata.owner == game.community_wallet
  )]
  pub community_treasury_ata: Account<'info, TokenAccount>,
  #[account(
      mut,
      seeds=[
          PLAYER_SEED_PREFIX.as_bytes(),
          payer.key().as_ref(),
          player.game.as_ref(),            
      ],
      bump = player.bump
  )]
  pub player: Account<'info, Player>,
  pub token_program: Program<'info, Token>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Claim<'info>
{
    #[account(mut)]
    pub claimer: Signer<'info>,
    #[account(mut)]
    pub claimer_ata: Account<'info, TokenAccount>,
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
    #[account(mut)]
    pub game_treasury_ata: Account<'info, TokenAccount>,
    #[account(
      mut,
      seeds=[
          PLAYER_SEED_PREFIX.as_bytes(),
          claimer.key().as_ref(),
          player.game.as_ref(),            
      ],
      bump = player.bump
    )]
    pub player: Account<'info, Player>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Withdraw<'info>
{
    #[account(mut)]
    pub claimer: Signer<'info>,
    #[account(mut)]
    pub claimer_ata: Account<'info, TokenAccount>,
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
    #[account(mut)]
    pub game_treasury_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}
