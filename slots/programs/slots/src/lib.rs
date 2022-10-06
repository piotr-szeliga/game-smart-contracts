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

    pub fn create_game(ctx: Context<CreateGame>, name: String, bump: u8, token_type: bool, community_wallet: Pubkey) -> Result<()> {
        let index = APPROVED_WALLETS.iter().any(|x| x.parse::<Pubkey>().unwrap() == ctx.accounts.payer.key());
        if index == false {
            return Err(ErrorCode::UnauthorizedWallet.into());
        }

        let game = &mut ctx.accounts.game;
        game.authority = ctx.accounts.payer.key();
        game.name = name;
        game.bump = bump;
        game.token_type = token_type;
        game.community_wallet = community_wallet;
        game.royalty = 5;
        game.main_balance = 0;
        game.community_balance = 0;
        game.jackpot = 14_400_000_000;
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
        game.community_balance = 0;
        Ok(())
    }

    pub fn set_jackpot(ctx: Context<SetJackpot>, jackpot: u64) -> Result<()> {
        let index = APPROVED_WALLETS.iter().any(|x| x.parse::<Pubkey>().unwrap() == ctx.accounts.payer.key());
        if index == false {
            return Err(ErrorCode::UnauthorizedWallet.into());
        }

        let game = &mut ctx.accounts.game;
        game.jackpot = jackpot;
        Ok(())
    }

    pub fn add_player(ctx: Context<AddPlayer>, bump: u8) -> Result<()> {
        let player = &mut ctx.accounts.player;
        player.key = ctx.accounts.payer.key();
        player.game = ctx.accounts.game.key();
        player.earned_money = 0;
        player.bump = bump;
        player.status = 0;

        Ok(())
    }

    pub fn play(ctx: Context<Play>, price: u64) -> Result<()> {
        let game = &ctx.accounts.game;
        let player = &mut ctx.accounts.player;
        let jackpot = game.jackpot;
        let (rand, earned) = get_status(price, jackpot);
        player.status = rand;

        
        let royalty_amount = price.checked_mul(game.royalty as u64).unwrap().checked_div(100).unwrap();
        
        match game.token_type {
            false => {
                system_program::transfer(
                    CpiContext::new(
                        ctx.accounts.system_program.to_account_info(),
                        system_program::Transfer {
                            from: ctx.accounts.payer.to_account_info().clone(),
                            to: ctx.accounts.game.to_account_info(),
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
            },
            true => {
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
            }
        }
        
        let game = &mut ctx.accounts.game;
        game.main_balance = game.main_balance.checked_add(price).unwrap().checked_sub(royalty_amount).unwrap();
        game.community_balance = game.community_balance.checked_add(royalty_amount).unwrap();
        player.earned_money = player.earned_money.checked_add(earned).unwrap();
        
        Ok(())
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        let game = &ctx.accounts.game;
        let player = &mut ctx.accounts.player;
        
        let amount = player.earned_money;
        player.earned_money = 0;

        if game.token_type == false {
            **ctx.accounts.game.to_account_info().try_borrow_mut_lamports()? -= amount;
            **ctx.accounts.claimer.try_borrow_mut_lamports()? += amount;
        } else {
            let game_name = &game.name;
            let authority = game.authority;
            let seeds = [
                game_name.as_bytes(),
                GAME_SEED_PREFIX.as_bytes(),
                authority.as_ref(),
                &[game.bump]
            ];

            transfer(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    Transfer {
                        authority: ctx.accounts.game.to_account_info().clone(),
                        from: ctx.accounts.game_treasury_ata.to_account_info().clone(),
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
            **ctx.accounts.game.to_account_info().try_borrow_mut_lamports()? -= amount;
            **ctx.accounts.claimer.try_borrow_mut_lamports()? += amount;
        } else {
            let game_name = &game.name;
            let authority = game.authority;
            let seeds = [
                game_name.as_bytes(),
                GAME_SEED_PREFIX.as_bytes(),
                authority.as_ref(),
                &[game.bump]
            ];

            transfer(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    Transfer {
                        authority: ctx.accounts.game.to_account_info().clone(),
                        from: ctx.accounts.game_treasury_ata.to_account_info().clone(),
                        to: ctx.accounts.claimer_ata.to_account_info().clone(),
                    }
                ).with_signer(&[&seeds[..]]),
                amount,
            )?;
        }
     
        let game = &mut ctx.accounts.game;
        game.main_balance = game.main_balance.checked_sub(amount).unwrap();

        Ok(())
    }
}