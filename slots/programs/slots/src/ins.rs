use anchor_lang::prelude::*;
use crate::state::*;
use crate::constants::*;

#[derive(Accounts)]
pub struct InitializeGame<'info> 
{
    #[account(mut)]
    pub payer: Signer<'info>,
    // game
    #[account(init, payer = payer, space = Game::LEN + 8)]
    pub game: Account<'info, Game>,
    // game pool pda account 
    /// CHECK:
    #[account(init, payer = payer, space = 0, seeds = [GAME_POOL_SEED_PREFIX.as_bytes(), game.key().as_ref()], bump)]
    pub game_pool: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(round_no: u8)]
pub struct Run<'info>
{
    #[account(mut)]
    pub player: Signer<'info>,
    pub game: Account<'info, Game>,
    #[account(mut, seeds = [GAME_POOL_SEED_PREFIX.as_bytes(), game.key().as_ref()], bump = game.bump)]
    pub game_pool: SystemAccount<'info>,
    #[account(
        init, 
        payer = player, 
        space = Round::LEN + 8, 
        seeds=[
            &[round_no],
            player.key().as_ref(),
            game.key().as_ref(),            
        ],
        bump
    )]
    pub round: Account<'info, Round>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(round_no: u8)]
pub struct Claim<'info>
{
    /// CHECK:
    #[account(mut)]
    pub player: AccountInfo<'info>,
    pub game: Account<'info, Game>,
    #[account(mut, seeds = [GAME_POOL_SEED_PREFIX.as_bytes(), game.key().as_ref()], bump = game.bump)]
    pub game_pool: SystemAccount<'info>,
    #[account(
        seeds=[
            &[round_no],
            player.key().as_ref(),
            game.key().as_ref(),            
        ],
        bump
    )]
    pub round: Account<'info, Round>,
    pub system_program: Program<'info, System>
}
