use anchor_lang::prelude::*;
use anchor_lang::system_program::{self, Transfer as TransferProgramSOL};
use anchor_spl::token::{self, Transfer as TransferSPL};
use crate::state::{Raffle, BuyEvent, Buyer};
use crate::ins::*;
use crate::state::{ErrorCode};
use crate::utils::*;
use crate::id;

pub const LAMPORTS_PER_SOL: u64 = 1000000000;

pub fn initialize(ctx: Context<Initialize>, token_spl_address: Pubkey, ticket_price: u64, amount: u32) -> Result<()>
{
    let raffle = &mut ctx.accounts.raffle;

    raffle.token_spl_address = token_spl_address;
    raffle.price_per_ticket = ticket_price;
    raffle.total_tickets = amount;
    raffle.sold_tickets = 0;

    if ctx.accounts.sender_tokens.amount.clone() < 1 as u64
    {
        return err!(ErrorCode::NotEnoughTokens);
    }

    /* Option A: */
    transfer_spl_token(
        Context::new
            (
                &id(),
                &mut TransferSPLToken
                {
                    sender: ctx.accounts.payer.clone(),
                    sender_tokens: ctx.accounts.sender_tokens.clone(),
                    recipient_tokens: ctx.accounts.recipient_tokens.clone(),
                    token_program: ctx.accounts.token_program.clone()
                },
                &[],
                ctx.bumps.clone()
            )
    )?;

    msg!("Program initialized successfully.");
    msg!("Total Tickets: {:?}", raffle.total_tickets);
    msg!("Sold Tickets: {:?}", raffle.sold_tickets);
    msg!("Price Per Ticket: {} {}", raffle.price_per_ticket, raffle.price_per_ticket as f64 / LAMPORTS_PER_SOL as f64);
    msg!("Token SPL Address: {:?}", raffle.token_spl_address);
    msg!("New Raffle Account: {}", ctx.accounts.raffle.to_account_info().key());

    Ok(())
}

pub fn update_raffle(raffle: &mut Raffle, buyer: Pubkey, amount: u32) -> Result<()> {
    raffle.sold_tickets = raffle.sold_tickets.checked_add(amount).unwrap();
    let transaction_price = raffle.price_per_ticket * amount as u64;

    emit!(BuyEvent
        {
            buyer: buyer,
            amount: amount,
            sold_tickets: raffle.sold_tickets,
            total_tickets: raffle.total_tickets,
            remaining_tickets: raffle.total_tickets.checked_sub(raffle.sold_tickets).unwrap()
        });


    let remaining_tickets = raffle.total_tickets.checked_sub(raffle.sold_tickets).unwrap();

    let index = raffle.buyers.iter().position(|x| x.key == buyer);
    if let Some(index) = index {
        let item = &mut raffle.buyers[index];
        item.tickets = item.tickets.checked_add(amount).unwrap();
    } else {
        let item = Buyer {
            key: buyer,
            tickets: amount,
        };
        raffle.buyers.push(item);
    }

    msg!("Buyer: {:?}", buyer);
    msg!("Total Tickets: {:?} | Sold {:?} | Remaining: {:?} | Price {:?} ({})", raffle.total_tickets, raffle.sold_tickets, remaining_tickets, raffle.price_per_ticket, raffle.price_per_ticket as f64 / LAMPORTS_PER_SOL as f64);
    msg!("Buy Amount: {:?} | Total Cost: {:?} ({})", amount, transaction_price, transaction_price as f64 / LAMPORTS_PER_SOL as f64);

    Ok(())
}

pub fn buy_ticket_sol(ctx: Context<BuyTicketSOL>, amount: u32, _ticket_price: u64, _token_spl_address: Pubkey) -> Result<()>
{
    let raffle = &mut ctx.accounts.raffle;
    let transaction_price = raffle.price_per_ticket * amount as u64;

    msg!("SOL Transfer: {:?}", raffle.token_spl_address.key());

    // transfer via SOL
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            TransferProgramSOL {
                from: ctx.accounts.buyer.to_account_info().clone(),
                to: ctx.accounts.recipient.to_account_info(),
            },
        ),
        transaction_price,
    )?;

    update_raffle(raffle, ctx.accounts.buyer.key(), amount)
}

pub fn buy_ticket_spl(ctx: Context<BuyTicketSPL>, amount: u32, _ticket_price: u64, _token_spl_address: Pubkey) -> Result<()>
{
    let raffle = &mut ctx.accounts.raffle;
    let transaction_price = raffle.price_per_ticket * amount as u64;

    msg!("SPL-Token Transfer: {:?}", raffle.token_spl_address.key());

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferSPL {
                from: ctx.accounts.sender_tokens.to_account_info(),
                to: ctx.accounts.recipient_tokens.to_account_info(),
                authority: ctx.accounts.sender.to_account_info(),
            },
        ),
        transaction_price,
    )?;

    msg!("Token Type: {:?}", raffle.token_spl_address.key());

    update_raffle(raffle, ctx.accounts.sender.key(), amount)
}