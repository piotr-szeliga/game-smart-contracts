use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Transfer};
use anchor_lang::solana_program::program::invoke;
use spl_memo::build_memo;

use crate::ins::*;
use crate::state::{Raffle, BuyEvent, Buyer};
use crate::raffle::LAMPORTS_PER_SOL;

pub fn transfer_spl_token(ctx: Context<TransferSPLToken>) -> Result<()>
{
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.sender_tokens.to_account_info(),
                to: ctx.accounts.recipient_tokens.to_account_info(),
                authority: ctx.accounts.sender.to_account_info(),
            },
        ),
        1,
    )?;

    msg!("Transfer {} Done!",  ctx.accounts.recipient_tokens.mint);
    msg!("System ID {}!",  &System::id());

    Ok(())
}

pub fn memo(ctx: Context<Memo>) -> Result<()> {
    let account_info = vec![
        ctx.accounts.memo.to_account_info()
    ];

    invoke(
        &build_memo("Hello world".as_bytes(), &[]),
        account_info.as_slice()
    )?;

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