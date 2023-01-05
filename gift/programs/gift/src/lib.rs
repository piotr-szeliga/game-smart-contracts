mod state;
mod ins;

use anchor_lang::{
    prelude::*,
    solana_program::program::invoke
};
use anchor_spl::token::{MintTo, mint_to, Transfer,  transfer, Burn, burn};
use mpl_token_metadata::{
    instruction::create_metadata_accounts_v2,
    state::Creator,
};

use crate::ins::*;
use crate::state::{ErrorCode};


declare_id!("4NtgP5gmSJFPoifmXS1Fw1zHGvwhguRNhzT5jP7u3U6A");

#[program]
pub mod gift {
    use super::*;

    pub fn create_gift(ctx: Context<CreateGift>, token_amount: u64, name: String, symbol: String, uri: String) -> Result<()> {
        let gift = &mut ctx.accounts.gift;

        gift.bump = *ctx.bumps.get("gift").unwrap();
        gift.creator = ctx.accounts.creator.key();
        gift.spl_token_mint = ctx.accounts.spl_token_mint.key();
        gift.token_amount = token_amount;
        gift.nft_mint = ctx.accounts.nft_mint.key();
        gift.destination_address = ctx.accounts.target.key();
        gift.redeemed = false;

        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.creator_token_ata.to_account_info(),
                to: ctx.accounts.gift_token_ata.to_account_info(),
                authority: ctx.accounts.creator.to_account_info(),
            }
        );
        transfer(cpi_context, token_amount)?;

        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.nft_mint.to_account_info(),
                to: ctx.accounts.target_nft_ata.to_account_info(),
                authority: ctx.accounts.target.to_account_info(),
            }
        );
        let result =  mint_to(cpi_context, 1);
        
        if let Err(_) = result {
            return Err(ErrorCode::MintFailed.into());
        }
        msg!("Token Minted!");
        
        msg!("Metadata account creating:");

        let accounts = vec![
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.nft_mint.to_account_info(),
            ctx.accounts.creator.to_account_info(), // payer
            ctx.accounts.rent.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ];
        let creators = vec![
           Creator {
              address: ctx.accounts.gift.key(),
              verified: false,
              share: 0
            },
           Creator {
              address: ctx.accounts.creator.key(),
              verified: false,
              share: 100
            }
        ];
        
        let result = invoke(
            &create_metadata_accounts_v2(
                ctx.accounts.token_metadata_program.key(),
                ctx.accounts.metadata.key(),
                ctx.accounts.nft_mint.key(),
                ctx.accounts.creator.key(), // mint authority
                ctx.accounts.creator.key(), // payer
                ctx.accounts.creator.key(), // update authority
                name,
                symbol,
                uri,
                Some(creators),
                0,
                true,
                false,
                None,
                None,
            ),
            &accounts
        );
        if let Err(_) = result {
            return Err(ErrorCode::MetadataCreateFailed.into());
        }
        msg!("Metadata account created !!!");

        Ok(())
    }

    pub fn redeem(ctx: Context<Redeem>) -> Result<()> {
        let gift = &ctx.accounts.gift;

        require!(gift.redeemed == false, ErrorCode::AlreadyRedeemed);
        
        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.nft_mint.to_account_info(),
                from: ctx.accounts.target_nft_ata.to_account_info(),
                authority: ctx.accounts.target.to_account_info(),
            }
        );

        burn(cpi_context, 1)?;

        let nft_mint = gift.nft_mint;
        let bump = gift.bump;
        let seeds = [
            b"gift".as_ref(),
            nft_mint.as_ref(),
            &[bump]
        ];
        let signer = &[&seeds[..]];

        let cpi_context = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.gift_token_ata.to_account_info(),
                to: ctx.accounts.target_token_ata.to_account_info(),
                authority: ctx.accounts.gift.to_account_info(),
            },
            signer
        );

        transfer(cpi_context, gift.token_amount)?;

        let gift = &mut ctx.accounts.gift;
        gift.redeemed = true;

        Ok(())
    }
}

