use anchor_lang::prelude::*;
use anchor_lang::system_program::{self, Transfer as TransferProgramSOL};
use anchor_spl::token::{self, Transfer as TransferSPL};
use crate::state::{Raffle, BuyEvent, Buyer};
use crate::ins::*;
use crate::state::{ErrorCode};
use crate::constants::*;

pub fn initialize(ctx: Context<Initialize>, token_spl_address: Pubkey, ticket_price: u64, amount: u32, store_buyers: bool) -> Result<()>
{
    let raffle = &mut ctx.accounts.raffle;

    raffle.owner = ctx.accounts.payer.key();
    raffle.token_spl_address = token_spl_address;
    raffle.price_per_ticket = ticket_price;
    raffle.total_tickets = amount;
    raffle.sold_tickets = 0;
    raffle.store_buyers = store_buyers;
    raffle.buyers = vec![];

    if ctx.accounts.sender_ata.amount.clone() < 1 as u64
    {
        return err!(ErrorCode::NotEnoughTokens);
    }

    // Transfer NFT to raffle bank
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferSPL {
                from: ctx.accounts.sender_ata.to_account_info(),
                to: ctx.accounts.raffle_pool_ata.to_account_info(),
                authority: ctx.accounts.payer.to_account_info(),
            },
        ),
        1,
    )?;

    msg!("Program initialized successfully with Bank Vault.");
    msg!("Total Tickets: {:?}", raffle.total_tickets);
    msg!("Sold Tickets: {:?}", raffle.sold_tickets);
    msg!("Price Per Ticket: {} {}", raffle.price_per_ticket, raffle.price_per_ticket as f64 / LAMPORTS_PER_SOL as f64);
    msg!("Token SPL Address: {:?}", raffle.token_spl_address);
    msg!("Store Buyers: {:?}", raffle.store_buyers);
    msg!("New Raffle Account: {}", ctx.accounts.raffle.to_account_info().key());

    Ok(())
}

pub fn initialize_with_pda(ctx: Context<InitializeWithPDA>, pool_bump: u8, token_spl_address: Pubkey, ticket_price: u64, amount: u32, store_buyers: bool) -> Result<()>
{
    let raffle = &mut ctx.accounts.raffle;

    raffle.pool_bump = pool_bump;
    raffle.token_spl_address = token_spl_address;
    raffle.price_per_ticket = ticket_price;
    raffle.total_tickets = amount;
    raffle.sold_tickets = 0;
    raffle.store_buyers = store_buyers;
    raffle.buyers = vec![];

    if ctx.accounts.sender_ata.amount.clone() < 1 as u64
    {
        return err!(ErrorCode::NotEnoughTokens);
    }

    /* Option A: */
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferSPL {
                from: ctx.accounts.sender_ata.to_account_info(),
                to: ctx.accounts.raffle_pool_ata.to_account_info(),
                authority: ctx.accounts.payer.to_account_info(),
            },
        ),
        1,
    )?;

    msg!("Program initialized successfully with PDA.");
    msg!("Total Tickets: {:?}", raffle.total_tickets);
    msg!("Sold Tickets: {:?}", raffle.sold_tickets);
    msg!("Price Per Ticket: {} {}", raffle.price_per_ticket, raffle.price_per_ticket as f64 / LAMPORTS_PER_SOL as f64);
    msg!("Token SPL Address: {:?}", raffle.token_spl_address);
    msg!("Store Buyers: {:?}", raffle.store_buyers);
    msg!("Raffle Pool ATA: {:?}", ctx.accounts.raffle_pool_ata.to_account_info().key);
    msg!("New Raffle Account: {}", ctx.accounts.raffle.to_account_info().key());

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

    msg!("Token Type: SOL");

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

pub fn update_raffle(raffle: &mut Raffle, buyer: Pubkey, amount: u32) -> Result<()>
{
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

    // store the buyer if feature is enabled
    if raffle.store_buyers == true
    {
        let index = raffle.buyers.iter().position(|x| x.key == buyer);
        if let Some(index) = index
        {
            let item = &mut raffle.buyers[index];
            item.tickets = item.tickets.checked_add(amount).unwrap();
        }
        else
        {
            let item = Buyer
            {
                key: buyer,
                tickets: amount,
            };
            raffle.buyers.push(item);
        }
    }

    msg!("Buyer: {:?}", buyer);
    msg!("Total Tickets: {:?} | Sold {:?} | Remaining: {:?} | Price {:?} ({})", raffle.total_tickets, raffle.sold_tickets, remaining_tickets, raffle.price_per_ticket, raffle.price_per_ticket as f64 / LAMPORTS_PER_SOL as f64);
    msg!("Buy Amount: {:?} | Total Cost: {:?} ({})", amount, transaction_price, transaction_price as f64 / LAMPORTS_PER_SOL as f64);

    Ok(())
}

// Withdraw tokens from PDA
pub fn withdraw_from_pda(ctx: Context<WithdrawFromPDA>, amount: u64) -> Result<()> 
{
    let global = &ctx.accounts.global;
    if global.authorized_admins.iter().any(|x| x == &ctx.accounts.admin.key()) == false {
        return Err(ErrorCode::NotAuthorizedAdmin.into());
    }

    let raffle = &ctx.accounts.raffle;
    let raffle_address = ctx.accounts.raffle.key().clone();
    let seeds = [
        RAFFLE_POOL_SEED_PREFIX.as_bytes(),
        raffle_address.as_ref(),
        &[raffle.pool_bump],
    ];

    // Transfer token from PDA pool to user ATA
    let cpi_context = CpiContext::new(
        ctx.accounts.token_program.to_account_info().clone(),
        TransferSPL {
            from: ctx.accounts.raffle_pool_ata.to_account_info().clone(),
            to: ctx.accounts.dst_ata.to_account_info().clone(),
            authority: ctx.accounts.raffle_pool.to_account_info().clone(),
        }
    );

    // Transfer the token
    token::transfer(cpi_context.with_signer(&[&seeds[..]]), amount)?;

    // If no more balance in token account, let's close it
    if ctx.accounts.raffle_pool_ata.amount == 0
    {
        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::CloseAccount {
                account: ctx.accounts.raffle_pool_ata.to_account_info().clone(),
                destination: ctx.accounts.admin.to_account_info().clone(),
                authority: ctx.accounts.raffle_pool.to_account_info().clone()
            }
        );
        token::close_account(cpi_context)?;
    }

    Ok(())
}

pub fn raffle_finalize(ctx: Context<RaffleFinalize>, raffle_royalties: u8) -> Result<()> 
{
    /* Transfer NFT to winner only if 'winner_nft_ata' is set */
    if ctx.accounts.winner_nft_ata.key() != Pubkey::default()
    {
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                TransferSPL {
                    from: ctx.accounts.raffle_nft_ata.to_account_info(),
                    to: ctx.accounts.winner_nft_ata.to_account_info(),
                    authority: ctx.accounts.raffle_bank.to_account_info(),
                },
            ),
            1,
        )?;
    }

    /* Transfer raffle winnings minus royalties */
    let raffle = &ctx.accounts.raffle;
    let mut amount = raffle.price_per_ticket.checked_mul(raffle.sold_tickets as u64).unwrap();

    // calculate royalties
    amount = amount.checked_mul(100).unwrap()
        .checked_sub
        (
            amount.checked_mul(raffle_royalties as u64).unwrap()
        )
        .unwrap()
        .checked_div(100).unwrap();

    if raffle.token_spl_address == Pubkey::default() // transfer winning via SOL
    {
        // transfer via SOL
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                TransferProgramSOL {
                    from: ctx.accounts.raffle_bank.to_account_info().clone(),
                    to: ctx.accounts.owner.to_account_info().clone(),
                },
            ),
            amount,
        )?;
    }
    else // transfer winning via SPL-Token
    {
        // transfer via SPL-Token
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                TransferSPL {
                    from: ctx.accounts.raffle_spl_ata.to_account_info(),
                    to: ctx.accounts.owner_spl_ata.to_account_info(),
                    authority: ctx.accounts.raffle_bank.to_account_info(),
                },
            ),
            amount,
        )?;
    }

    msg!("Winner NFT ATA: {:?}", ctx.accounts.winner_nft_ata);
    msg!("Raffle Royalties: {:?}", raffle_royalties);
    msg!("Raffle owner: {:?}", raffle.owner);
    msg!("Payment token Spl-Address: {:?}", raffle.token_spl_address);
    msg!("Payment amount sent to raffle owner: {:?}", amount);
    Ok(())
}