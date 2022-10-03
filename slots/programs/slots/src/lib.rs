mod ins;
mod state;
mod constants;
mod utils;

use anchor_lang::{
    prelude::*,
    system_program,
};
use anchor_spl::token::{transfer, Transfer};

use ins::*;
use utils::*;
use state::ErrorCode;
use constants::*;

declare_id!("DMMYkdhZQyKLegrBVw85jUvyHq5P6Gp6MnyUEmzvptCP");

#[program]
pub mod slots {
    use super::*;

    pub fn create_game(ctx: Context<CreateGame>, name: String, bump: u8, treasury_bump: u8, token_type: bool, community_wallet: Pubkey) -> Result<()> {
        let index = APPROVED_WALLETS.iter().any(|x| x.parse::<Pubkey>().unwrap() == ctx.accounts.payer.key());
        if index == false {
            return Err(ErrorCode::UnauthorizedWallet.into());
        }

        let game = &mut ctx.accounts.game;
        game.authority = ctx.accounts.payer.key();
        game.name = name;
        game.bump = bump;
        game.treasury_bump = treasury_bump;
        game.token_type = token_type;
        game.community_wallet = community_wallet;
        game.royalty = 5;
        game.earned_money = 0;
        Ok(())
    }


    pub fn set_community_wallet(ctx: Context<SetCommunityWallet>, community_wallet: Pubkey, royalty: u8) -> Result<()> {
        let index = APPROVED_WALLETS.iter().any(|x| x.parse::<Pubkey>().unwrap() == ctx.accounts.payer.key());
        if index == false {
            return Err(ErrorCode::UnauthorizedWallet.into());
        }

        let game = &mut ctx.accounts.game;
        game.community_wallet = community_wallet;
        game.royalty = royalty;
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

    pub fn play_with_sol(ctx: Context<PlayWithSol>, price: u64) -> Result<()> {
        let player = &mut ctx.accounts.player;
        let (rand, earned) = get_status(price);
        player.status = rand;

        let game = &mut ctx.accounts.game;
        if game.token_type != false {
            return Err(ErrorCode::InvalidTokenType.into());
        }
        let royalty_amount = price.checked_mul(game.royalty as u64).unwrap().checked_div(100).unwrap();
        
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.payer.to_account_info().clone(),
                    to: ctx.accounts.game_treasury.to_account_info(),
                },
            ),
            price.checked_sub(royalty_amount).unwrap(),
        )?;
        if royalty_amount > 0 {
            system_program::transfer(
                CpiContext::new(
                    ctx.accounts.system_program.to_account_info(),
                    system_program::Transfer {
                        from: ctx.accounts.payer.to_account_info().clone(),
                        to: ctx.accounts.community_treasury.to_account_info(),
                    },
                ),
                royalty_amount,
            )?;
        }
        game.earned_money = game.earned_money.checked_add(price).unwrap().checked_sub(royalty_amount).unwrap();

        if earned > 0 {
            player.earned_money = player.earned_money.checked_add(earned).unwrap();

            **ctx.accounts.game_treasury.try_borrow_mut_lamports()? -= earned;
            **ctx.accounts.player_treasury.try_borrow_mut_lamports()? += earned;
        }
        
        Ok(())
    }

    pub fn play_with_spl(ctx: Context<PlayWithSpl>, price: u64) -> Result<()> {
        let player = &mut ctx.accounts.player;
        let (rand, earned) = get_status(price);
        player.status = rand;

        let game = &mut ctx.accounts.game;
        if game.token_type != true {
            return Err(ErrorCode::InvalidTokenType.into());
        }
        let royalty_amount = price.checked_mul(game.royalty as u64).unwrap().checked_div(100).unwrap();
        
        transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                        authority: ctx.accounts.payer.to_account_info().clone(),
                        from: ctx.accounts.payer_ata.to_account_info().clone(),
                    to: ctx.accounts.game_treasury_ata.to_account_info().clone(),
                }
            ),
            price.checked_sub(royalty_amount).unwrap(),
        )?;
        if royalty_amount > 0 {
            transfer(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    Transfer {
                        authority: ctx.accounts.payer.to_account_info().clone(),
                        from: ctx.accounts.payer_ata.to_account_info().clone(),
                        to: ctx.accounts.community_treasury_ata.to_account_info().clone(),
                    }
                ),
                royalty_amount,
            )?;
        }
        game.earned_money = game.earned_money.checked_add(price).unwrap().checked_sub(royalty_amount).unwrap();

        if earned > 0 {
            player.earned_money = player.earned_money.checked_add(earned).unwrap();

            let game_key = game.key();
            let seeds = [
                GAME_TREASURY_SEED_PREFIX.as_bytes(),
                game_key.as_ref(),
                &[game.treasury_bump]
            ];
            transfer(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    Transfer {
                        authority: ctx.accounts.game_treasury.to_account_info().clone(),
                        from: ctx.accounts.game_treasury_ata.to_account_info().clone(),
                        to: ctx.accounts.player_treasury_ata.to_account_info().clone(),
                    }
                ).with_signer(&[&seeds[..]]),
                earned,
            )?;
        }
        Ok(())
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        let game = &ctx.accounts.game;
        let player = &mut ctx.accounts.player;
        
        let amount = player.earned_money;
        player.earned_money = 0;

        if game.token_type == false {
            **ctx.accounts.player_treasury.try_borrow_mut_lamports()? -= amount;
            **ctx.accounts.claimer.try_borrow_mut_lamports()? += amount;
        } else {
            let player_key = player.key;
            let seeds = [
                PLAYER_TREASURY_SEED_PREFIX.as_bytes(),
                player_key.as_ref(),
                &[player.treasury_bump]
            ];

            transfer(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    Transfer {
                        authority: ctx.accounts.player_treasury.to_account_info().clone(),
                        from: ctx.accounts.player_treasury_ata.to_account_info().clone(),
                        to: ctx.accounts.claimer_ata.to_account_info().clone(),
                    }
                ).with_signer(&[&seeds[..]]),
                amount,
            )?;
        }
        
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let index = APPROVED_WALLETS.iter().any(|x| x.parse::<Pubkey>().unwrap() == ctx.accounts.claimer.key());
        if index == false {
            return Err(ErrorCode::UnauthorizedWallet.into());
        }

        let game = &ctx.accounts.game;
        
        if game.token_type == false {
            **ctx.accounts.game_treasury.try_borrow_mut_lamports()? -= amount;
            **ctx.accounts.claimer.try_borrow_mut_lamports()? += amount;
        } else {
            let game_key = game.key();
            let seeds = [
                GAME_TREASURY_SEED_PREFIX.as_bytes(),
                game_key.as_ref(),
                &[game.treasury_bump]
            ];

            transfer(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    Transfer {
                        authority: ctx.accounts.game_treasury.to_account_info().clone(),
                        from: ctx.accounts.game_treasury_ata.to_account_info().clone(),
                        to: ctx.accounts.claimer_ata.to_account_info().clone(),
                    }
                ).with_signer(&[&seeds[..]]),
                amount,
            )?;
        }
        
        Ok(())
    }
}