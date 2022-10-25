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

    pub fn create_game(
        ctx: Context<CreateGame>, 
        name: String, 
        bump: u8, 
        token_type: bool, 
        community_wallets: Vec<Pubkey>, 
        royalties: Vec<u16>,
        commission_wallet: Pubkey,
        commission_fee: u16,
    ) -> Result<()> {
        let index = APPROVED_WALLETS.iter().any(|x| x.parse::<Pubkey>().unwrap() == ctx.accounts.payer.key());
        if index == false {
            return Err(ErrorCode::UnauthorizedWallet.into());
        }

        let game = &mut ctx.accounts.game;
        game.authority = ctx.accounts.payer.key();
        game.name = name;
        game.bump = bump;
        game.token_type = token_type;
        game.community_wallets = community_wallets;
        game.commission_wallet = commission_wallet;
        game.commission_fee = commission_fee;
        let len = royalties.len();
        game.royalties = royalties;
        game.main_balance = 0;
        game.community_balances = vec![0; len]; 
        game.community_pending_balances = vec![0; len];
        game.jackpot = 14_400_000_000;        
        game.win_percents = [
            [2500, 1500, 0],
            [2000, 1000, 0],
            [1500,  500, 0],
            [1000,  250, 0],
            [ 500,  200, 0],
            [ 200,  100, 0],
        ];
        game.min_rounds_before_win = 4;
        game.lose_counter = 4;
        Ok(())
    }


    pub fn set_community_wallet(ctx: Context<ConfigGame>, community_wallet: Pubkey, royalty: u16) -> Result<()> {
        let index = APPROVED_WALLETS.iter().any(|x| x.parse::<Pubkey>().unwrap() == ctx.accounts.payer.key());
        if index == false {
            return Err(ErrorCode::UnauthorizedWallet.into());
        }

        let game = &mut ctx.accounts.game;
        
        let index = game.community_wallets.iter().position(|x| x == &community_wallet);
        if let Some(index) = index {
            game.royalties[index] = royalty;
            if royalty == 10001 {
                game.royalties.remove(index);
                game.community_balances.remove(index);
                game.community_wallets.remove(index);
                game.community_pending_balances.remove(index);
                msg!("Removed");
            } else {
                msg!("Updated");
                msg!("New Royalty: {:?}", royalty);
            }
        } else {
            game.community_wallets.push(community_wallet);
            game.royalties.push(royalty);
            game.community_balances.push(0);
            game.community_pending_balances.push(0);
            msg!("New Added");
        }
        msg!("Community Wallet: {:?}", community_wallet);
        Ok(())
    }

    pub fn set_commission(ctx: Context<ConfigGame>, commission_wallet: Pubkey, commission_fee: u16) -> Result<()> {
        let index = APPROVED_WALLETS.iter().any(|x| x.parse::<Pubkey>().unwrap() == ctx.accounts.payer.key());
        if index == false {
            return Err(ErrorCode::UnauthorizedWallet.into());
        }

        let game = &mut ctx.accounts.game;
        game.commission_wallet = commission_wallet;
        game.commission_fee = commission_fee;

        Ok(())
    }

    pub fn set_winning(ctx: Context<ConfigGame>, win_percents: [[u16; 3]; 6], jackpot: u64, min_rounds_before_win: u8) -> Result<()> {
        let index = APPROVED_WALLETS.iter().any(|x| x.parse::<Pubkey>().unwrap() == ctx.accounts.payer.key());
        if index == false {
            return Err(ErrorCode::UnauthorizedWallet.into());
        }

        let game = &mut ctx.accounts.game;
        game.win_percents = win_percents;
        game.jackpot = jackpot;
        game.min_rounds_before_win = min_rounds_before_win;
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

    pub fn play(ctx: Context<Play>, bet_no: u8) -> Result<()> {
        if bet_no > 5 {
            return Err(ErrorCode::MinimumPrice.into());
        }
        let game = &ctx.accounts.game;
        let player = &mut ctx.accounts.player;
        let jackpot = game.jackpot;
        let win_percents = game.win_percents;
        let price = BET_PRICES[bet_no as usize];
        let (rand, earned) = get_status(bet_no, win_percents, jackpot, game.lose_counter < game.min_rounds_before_win);
        player.status = rand;

        let commission_amount = price.checked_mul(game.commission_fee as u64).unwrap().checked_div(10000).unwrap();
        msg!("Commission Amount: {:?}", commission_amount);
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
                    price.checked_sub(commission_amount).unwrap(),
                )?;
                if commission_amount > 0 {
                    system_program::transfer(
                        CpiContext::new(
                            ctx.accounts.system_program.to_account_info(),
                            system_program::Transfer {
                                from: ctx.accounts.payer.to_account_info().clone(),
                                to: ctx.accounts.commission_treasury.to_account_info(),
                            },
                        ),
                        commission_amount
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
                    price.checked_sub(commission_amount).unwrap(),
                )?;
                if commission_amount > 0 {
                    transfer(
                        CpiContext::new(
                            ctx.accounts.token_program.to_account_info(),
                            Transfer {
                                authority: ctx.accounts.payer.to_account_info().clone(),
                                from: ctx.accounts.payer_ata.to_account_info().clone(),
                                to: ctx.accounts.commission_treasury_ata.to_account_info().clone(),
                            }
                        ),
                        price.checked_sub(commission_amount).unwrap(),
                    )?;
                }
            }
        }
        
        let game = &mut ctx.accounts.game;
        game.lose_counter = game.lose_counter.checked_add(1).unwrap();
        if earned > 0 {
            game.lose_counter = 0;
        }
        game.main_balance = game.main_balance.checked_add(price).unwrap().checked_sub(commission_amount).unwrap();
        let len = game.royalties.len();
        for i in 0..len {
            let royalty = game.royalties[i];
            let royalty_amount = price.checked_mul(royalty as u64).unwrap().checked_div(10000).unwrap();
            game.community_pending_balances[i] = game.community_pending_balances[i].checked_add(royalty_amount).unwrap();
            game.main_balance = game.main_balance.checked_sub(royalty_amount).unwrap();
        }
        player.earned_money = player.earned_money.checked_add(earned).unwrap();
        
        Ok(())
    }

    pub fn send_to_community_wallet(ctx: Context<SendToCommunityWallet>) -> Result<()> {
        let game = &ctx.accounts.game;
        let index = game.community_wallets.iter().position(|x| x == &ctx.accounts.community_wallet.key());
        if let Some(index) = index {
            let amount = game.community_pending_balances[index];            
            if game.token_type == false {
                **ctx.accounts.game.to_account_info().try_borrow_mut_lamports()? -= amount;
                **ctx.accounts.community_wallet.try_borrow_mut_lamports()? += amount;
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
                            to: ctx.accounts.community_treasury_ata.to_account_info().clone(),
                        }
                    ).with_signer(&[&seeds[..]]),
                    amount,
                )?;
            }
            let game = &mut ctx.accounts.game;
            game.community_pending_balances[index] = 0;
            game.community_balances[index] = game.community_balances[index].checked_add(amount).unwrap();
        }
        Ok(())
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        let game = &ctx.accounts.game;
        let player = &mut ctx.accounts.player;
        
        let amount = player.earned_money;
        player.earned_money = 0;

        if game.token_type == false {
            msg!("Amount: {:?}", amount);
            **ctx.accounts.game.to_account_info().try_borrow_mut_lamports()? -= amount;
            **ctx.accounts.claimer.to_account_info().try_borrow_mut_lamports()? += amount;
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

    pub fn fund(ctx: Context<Fund>, amount: u64) -> Result<()> {
        let game = &mut ctx.accounts.game;
        game.main_balance = game.main_balance.checked_add(amount).unwrap();

        if game.token_type == false {
            system_program::transfer(
                CpiContext::new(
                    ctx.accounts.system_program.to_account_info(),
                    system_program::Transfer {
                        from: ctx.accounts.payer.to_account_info().clone(),
                        to: ctx.accounts.game.to_account_info(),
                    },
                ),
                amount,
            )?;
        } else {
            transfer(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    Transfer {
                        authority: ctx.accounts.payer.to_account_info().clone(),
                        from: ctx.accounts.payer_ata.to_account_info().clone(),
                        to: ctx.accounts.game_treasury_ata.to_account_info().clone(),
                    }
                ),
                amount,
            )?;
        }

        Ok(())
    }
}