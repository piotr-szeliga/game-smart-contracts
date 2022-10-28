mod ins;
mod state;
mod utils;
mod constants;

use anchor_lang::{prelude::* , system_program};
use anchor_spl::token::{self, Token, TokenAccount, Transfer, CloseAccount, close_account, transfer};

use crate::ins::*;
use crate::constants::*;
use crate::utils::*;
use crate::state::{ErrorCode, Game, Cell};

declare_id!("HsDXU6Ku6ggbqDzhxyrNLjby5MLdT7RXXnR6AhFChD7L");

#[program]
pub mod wheel {
    use super::*;

    pub fn initialize_game(ctx: Context<InitializeGame>, name: String, bump: u8) -> Result<()> {
        let game = &mut ctx.accounts.game;
        game.authority = *ctx.accounts.payer.key;
        game.bump = bump;
        game.name = name;

        Ok(())
    }

    #[access_control(authorized_admin(&ctx.accounts.funder))]
    pub fn fund_prize(ctx: Context<FundPrize>, index: usize, amount: u64, chance: u16) -> Result<()> {
        let game = &mut ctx.accounts.game;
        let cell = Cell {
            spl_mint: ctx.accounts.prize_mint.key(),
            amount,
            chance
        };
        game.cells[index] = cell;

        if cell.spl_mint == Pubkey::default() {
            system_program::transfer(
                CpiContext::new(
                    ctx.accounts.system_program.to_account_info(),
                    system_program::Transfer {
                        from: ctx.accounts.funder.to_account_info().clone(),
                        to: ctx.accounts.game.to_account_info(),
                    },
                ),
                amount,
            )?;
        } else {
            transfer(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info().clone(),
                    Transfer {
                        authority: ctx.accounts.funder.to_account_info().clone(),
                        from: ctx.accounts.funder_ata.to_account_info().clone(),
                        to: ctx.accounts.game_ata.to_account_info().clone(),
                    }
                ),
                amount,
            )?;

            if token::accessor::amount(&ctx.accounts.game_ata).unwrap() == 0 {
                anchor_spl::token::close_account(
                    CpiContext::new(
                        ctx.accounts.token_program.to_account_info().clone(), 
                        CloseAccount {
                            account: ctx.accounts.funder_ata.to_account_info().clone(),
                            destination: ctx.accounts.funder.to_account_info().clone(),
                            authority: ctx.accounts.funder.to_account_info().clone(),                        
                        }
                    )
                )?;
            }
        }
        Ok(())
    }

    #[access_control(authorized_admin(&ctx.accounts.withdrawer))]
    pub fn withdraw_prize(ctx: Context<WithdrawPrize>, index: usize) -> Result<()> {
        let game = &ctx.accounts.game;
        let amount = game.cells[index].amount;

        if ctx.accounts.game_ata.key() == Pubkey::default() {
            **ctx.accounts.game.to_account_info().try_borrow_mut_lamports()? -= amount;
            **ctx.accounts.withdrawer.to_account_info().try_borrow_mut_lamports()? += amount;
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
                    ctx.accounts.token_program.to_account_info().clone(),
                    Transfer {
                        authority: ctx.accounts.game.to_account_info().clone(),
                        from: ctx.accounts.game_ata.to_account_info().clone(),
                        to: ctx.accounts.withdrawer_ata.to_account_info().clone(),
                    }
                ).with_signer(&[&seeds[..]]),
                amount,
            )?;

            if token::accessor::amount(&ctx.accounts.game_ata).unwrap() == 0 {
                anchor_spl::token::close_account(
                    CpiContext::new(
                        ctx.accounts.token_program.to_account_info().clone(), 
                        CloseAccount {
                            account: ctx.accounts.game_ata.to_account_info().clone(),
                            destination: ctx.accounts.withdrawer.to_account_info().clone(),
                            authority: ctx.accounts.game.to_account_info().clone(),                        
                        }
                    ).with_signer(&[&seeds[..]])
                )?;
            }
        }
        let game = &mut ctx.accounts.game;        
        game.cells[index] = Cell::default();

        Ok(())
    }
}

pub fn authorized_admin(admin: &AccountInfo) -> Result<()> {
    let index = APPROVED_WALLETS.iter().any(|x| x.parse::<Pubkey>().unwrap() == admin.key());
    if index == false {
        return Err(ErrorCode::UnauthorizedWallet.into());
    }
    Ok(())
}