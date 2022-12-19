mod ins;
mod state;
mod constants;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;
use anchor_spl::token::{transfer, Transfer};

use crate::constants::*;
use crate::state::*;
use crate::ins::*;

declare_id!("73YJ31M6KX8FVH5g9mYEy7aHd3YYNFUsSP6EZCFe8iXo");

#[program]
pub mod auction {
    use super::*;

    pub fn test(ctx: Context<Test>) -> Result<()> {
        Ok(())
    }

    pub fn create_auction(
        ctx: Context<CreateAuction>, 
        auction_name: String, 
        min_bid_price: u64, 
        auction_duration: u64
    ) -> Result<()> {
        let auction = &mut ctx.accounts.auction;
        
        auction.name = auction_name;
        auction.creator = ctx.accounts.creator.key();
        auction.nft_mint = ctx.accounts.nft_mint.key();
        auction.spl_token_mint = ctx.accounts.spl_token_mint.key();
        auction.min_bid_price = min_bid_price;
        let now: u64 = Clock::get().unwrap().unix_timestamp.try_into().unwrap();
        auction.aution_started_time = now;
        auction.auction_finish_time = now.checked_add(auction_duration).unwrap();
        auction.last_bidder = Pubkey::default();
        auction.bump = *ctx.bumps.get(AUCTION_SEED).unwrap();

        Ok(())
    }

    pub fn bid(ctx: Context<Bid>, bid_price: u64) -> Result<()> {
        let auction = &ctx.accounts.auction;
        require!(bid_price > auction.min_bid_price, CustomError::MinBidPrice);

        let now: u64 = Clock::get().unwrap().unix_timestamp.try_into().unwrap();
        require!(now < auction.auction_finish_time, CustomError::AuctionFinished);

        let last_bid_price = auction.min_bid_price;
        let name = &auction.name;
        let creator = auction.creator;
        let bump = auction.bump;
        let seeds = &[
            AUCTION_SEED.as_ref(),
            name.as_ref(),
            creator.as_ref(),
            &[bump],
        ];
        let signer = &[&seeds[..]]; 

        let cpi_context = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
              authority: ctx.accounts.auction.to_account_info(),
              from: ctx.accounts.auction_token_ata.to_account_info(),
              to: ctx.accounts.last_bidder_ata.to_account_info(),
            },
            signer,
        );

        transfer(cpi_context, last_bid_price)?;

        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                authority: ctx.accounts.bidder.to_account_info(),
                from: ctx.accounts.bidder_ata.to_account_info(),
                to: ctx.accounts.auction_token_ata.to_account_info(),
            }
        );

        transfer(cpi_context, bid_price)?;

        let auction = &mut ctx.accounts.auction;

        
        auction.min_bid_price = bid_price;
        auction.last_bidder = ctx.accounts.bidder.key();

        Ok(())
    }

    pub fn transfer_to_winner(ctx: Context<TransferToWinner>) -> Result<()> {
        let auction = &ctx.accounts.auction;
        let now: u64 = Clock::get().unwrap().unix_timestamp.try_into().unwrap();
        require!(now > auction.auction_finish_time, CustomError::AuctionNotFinished);

        let name = &auction.name;
        let creator = auction.creator;
        let bump = auction.bump;
        let seeds = &[
            AUCTION_SEED.as_ref(),
            name.as_ref(),
            creator.as_ref(),
            &[bump],
        ];
        let signer = &[&seeds[..]]; 

        let cpi_context = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
              authority: ctx.accounts.auction.to_account_info(),
              from: ctx.accounts.auction_nft_ata.to_account_info(),
              to: ctx.accounts.winner_nft_ata.to_account_info(),
            },
            signer,
        );

        transfer(cpi_context, 1)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Test {

}