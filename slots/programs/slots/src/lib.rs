mod ins;
mod state;
mod constants;
mod utils;

use anchor_lang::{
    prelude::*,
    system_program,
};
use ins::*;
use utils::*;
use constants::*;

declare_id!("DMMYkdhZQyKLegrBVw85jUvyHq5P6Gp6MnyUEmzvptCP");

#[program]
pub mod slots {
    use super::*;

    pub fn initialize_game(ctx: Context<InitializeGame>, bump: u8) -> Result<()> {
        let game = &mut ctx.accounts.game;
        game.authroity = ctx.accounts.payer.key();
        game.bump = bump;
        Ok(())
    }

    pub fn run(ctx: Context<Run>, price: u64, round_no: u32) -> Result<()> {
        let round = &mut ctx.accounts.round;
        round.player = ctx.accounts.player.key();
        round.round_no = round_no;
        round.price = price;

        let mut rand = get_random();
        
        let mut status: [u8; 5] = [0; 5];
        let mut i = 0;
        while i < 5 {
            status[i] = ((rand % 5) & 0xff) as u8;
            rand /= 5;
            i += 1;
        }

        round.status = status;

        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.player.to_account_info().clone(),
                    to: ctx.accounts.game_pool.to_account_info(),
                },
            ),
            price,
        )?;
        Ok(())
    }

    pub fn claim(ctx: Context<Claim>, round_no: u8) -> Result<()> {
        let round = &mut ctx.accounts.round;
        let status = &round.status;
        let mut max_len = 1;
        let mut start = 0;
        let mut len = 0;
        while start + len < 5 {
            while status[start] == status[start + len] {
                len += 1;
            }
            if max_len < len {
                max_len = len;
            }
            start += len;
            len = 0;
        }
        let chance = (status[0] + status[4]) % 4;
        
        if max_len >= 3 {
            let game = &ctx.accounts.game;
            let game_address = ctx.accounts.game.key().clone();
            let seeds = [
                GAME_POOL_SEED_PREFIX.as_bytes(),
                game_address.as_ref(),
                &[game.bump]
            ];
            system_program::transfer(
                CpiContext::new(
                    ctx.accounts.system_program.to_account_info(),
                    system_program::Transfer {
                        from: ctx.accounts.game_pool.to_account_info().clone(),
                        to: ctx.accounts.player.to_account_info(),
                    },
                ).with_signer(&[&seeds[..]]),
                round.price * (2 + chance % 4) as u64,
            )?;
        }
        
        Ok(())
    }
}