use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod lootbox {
    use super::*;

    pub fn initialize_game(ctx: Context<InitializeGame>) -> Result<()> {
        Ok(())
    }

    pub fn run(ctx: Context<Run>, price: u64, rounde_no: u8) -> Result<()> {
        Ok(())
    }

    pub fn claim(ctx: Context<Claim>, round_no: u8) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeGame<'info> 
{
    #[account(mut)]
    pub payer: Signer<'info>,
    // game
    #[account(init, payer = payer, space = Game::LEN + 8)]
    pub game: Account<'info, Game>,
    // game pool pda account 
    #[account(init, payer = payer, space = 0, seeds = [GAME_POOL_SEED_PREFIX.as_bytes(), game.key().as_ref()], bump)]
    pub game_pool: SystemAccount<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(round_no: u8)]
pub struct Run<'info>
{
    #[account(mut)]
    pub player: Signer<'info>,
    pub game: Account<'info, Game>,
    #[account(
        init, 
        payer = player, 
        space = Round::LEN + 8, 
        seeds=[
            round_no,
            player.key().as_ref(),
            game.key().as_ref(),            
        ],
        bump
    )]
    pub round: Account<'info, Round>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(round_no: u8)]
pub struct Claim<'info>
{
    #[account(mut)]
    pub player: Signer<'info>,
    pub game: Account<'info, Game>,
    #[account(
        seeds=[
            round_no,
            player.key().as_ref(),
            game.key().as_ref(),            
        ],
        bump
    )]
    pub round: Account<'info, Round>,
    pub system_program: Program<'info, System>
}

#[account]
pub struct Game {
    pub authroity: Pubkey,
    pub bump: u8,
}
impl Game
{
    pub const LEN: usize =  std::mem::size_of::<Raffle>();
}

#[account]
pub struct Round {
    pub player: Pubkey,
    pub status: [u8; 5],
    pub price: u64,
}
impl Round
{
    pub const LEN: usize =  std::mem::size_of::<Raffle>();
}