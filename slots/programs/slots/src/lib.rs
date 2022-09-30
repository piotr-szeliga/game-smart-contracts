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

    pub fn create_game(ctx: Context<CreateGame>, name: String, bump: u8, treasury_bump: u8) -> Result<()> {
        let game = &mut ctx.accounts.game;
        game.authority = ctx.accounts.payer.key();
        game.name = name;
        game.bump = bump;
        game.treasury_bump = treasury_bump;
        Ok(())
    }

    pub fn add_player(ctx: Context<AddPlayer>, bump: u8, treasury_bump: u8) -> Result<()> {
        let player = &mut ctx.accounts.player;
        player.key = ctx.accounts.payer.key();
        player.game = ctx.accounts.game.key();
        player.earned_money = 0;
        player.bump = bump;
        player.treasury_bump = treasury_bump;
        player.status = 0;

        Ok(())
    }

    pub fn play(ctx: Context<Play>, price: u64) -> Result<()> {
        let player = &mut ctx.accounts.player;

        let mut rand = get_random();
        player.status = rand;
        
        let mut status: [u8; 5] = [0; 5];
        let mut i = 0;
        while i < 5 {
            status[i] = ((rand % 10) & 0xff) as u8;
            rand /= 10;
            i += 1;
        }

        let mut counts: [u8; 10] = [0; 10];
        i = 0;
        while i < 5 {
            counts[status[i] as usize] += 1;
            i += 1;
        }

        let mut max = 0;
        i = 0;
        while i < 10 {
            if max < counts[i] {
                max = counts[i];
            }
            i += 1;
        }

        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.payer.to_account_info().clone(),
                    to: ctx.accounts.game_treasury.to_account_info(),
                },
            ),
            price,
        )?;

        let max = 3;
        
        let earned = match max {
            3 => price,
            4 => price.checked_mul(5).unwrap().checked_div(4).unwrap(),
            5 => price.checked_mul(3).unwrap().checked_div(2).unwrap(),
            _ => 0,
        };
        
        if earned > 0 {
            player.earned_money = player.earned_money.checked_add(earned).unwrap();

            let game = &ctx.accounts.game;
            let game_key = game.key();
            let seeds = [
                GAME_TREASURY_SEED_PREFIX.as_bytes(),
                game_key.as_ref(),
                &[game.treasury_bump]
            ];
            system_program::transfer(
                CpiContext::new(
                    ctx.accounts.system_program.to_account_info(),
                    system_program::Transfer {
                        from: ctx.accounts.game_treasury.to_account_info().clone(),
                        to: ctx.accounts.player_treasury.to_account_info().clone(),
                    },
                ).with_signer(&[&seeds[..]]),
                earned,
            )?;
        }

        msg!("Status: {:?}", status);
        msg!("Max Equal: {:?}", max);
        
        Ok(())
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        let player = &mut ctx.accounts.player;
        let player_key = player.key;
        let seeds = [
            PLAYER_TREASURY_SEED_PREFIX.as_bytes(),
            player_key.as_ref(),
            &[player.treasury_bump]
        ];
        let amount = player.earned_money;
        player.earned_money = 0;
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.player_treasury.to_account_info().clone(),
                    to: ctx.accounts.claimer.to_account_info().clone(),
                },
            ).with_signer(&[&seeds[..]]),
            amount,
        )?;        
        Ok(())
    }
}