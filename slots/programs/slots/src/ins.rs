use anchor_lang::prelude::*;
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

    /// CHECK:
    #[account(
      init,
      payer = payer,
      space = 0,
      seeds = [
        GAME_TREASURY_SEED_PREFIX.as_bytes(),
        game.key().as_ref()
      ],
      bump
    )]
    pub game_treasury: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
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
  /// CHECK:
  #[account(
    init,
    payer = payer,
    space = 0,
    seeds = [
      PLAYER_TREASURY_SEED_PREFIX.as_bytes(),
      player.key().as_ref(),
    ],
    bump
  )]
  pub player_treasury: AccountInfo<'info>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Play<'info>
{
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
    /// CHECK:
    #[account(
      mut,
      seeds = [
        GAME_TREASURY_SEED_PREFIX.as_bytes(),
        game.key().as_ref()
      ],
      bump = game.treasury_bump
    )]
    pub game_treasury: AccountInfo<'info>,
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
    /// CHECK:
    #[account(
      mut,
      seeds = [
        PLAYER_TREASURY_SEED_PREFIX.as_bytes(),
        player.key().as_ref(),
      ],
      bump = player.treasury_bump
    )]
    pub player_treasury: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Claim<'info>
{
    #[account(mut)]
    pub claimer: Signer<'info>,
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
    /// CHECK:
    #[account(
      mut,
      seeds = [
        PLAYER_TREASURY_SEED_PREFIX.as_bytes(),
        player.key().as_ref(),
      ],
      bump = player.treasury_bump
    )]
    pub player_treasury: AccountInfo<'info>,
    pub system_program: Program<'info, System>
}
