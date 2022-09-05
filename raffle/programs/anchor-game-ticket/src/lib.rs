#![feature(core_intrinsics)]

mod ins;
mod state;

// use fmt::Debug;
use anchor_lang::prelude::*;
use ins::*;
use anchor_lang::system_program::{Transfer as TransferProgramSOL};
use anchor_spl::token::{self, Transfer as TransferSPL};
use crate::state::{ErrorCode, BuyEvent, Buyer};

//use std::fmt;
// use solana_sdk::{
//     signature::{Keypair, Signer},
//     transaction::{Transaction, TransactionError},
// };
// use spl_memo::*;

declare_id!("4coTofX2WZBYv1EA4Gr4N35687MH7iQj1sJSXuMrk2bV");

pub const LAMPORTS_PER_SOL: u64 = 1000000000;

#[program]
pub mod anchor_raffle_ticket
{
    //use std::str::FromStr;
    use anchor_lang::system_program;
    use anchor_spl::associated_token::{Create, create};
    use super::*;

    /* TEMP CODE:
    // use solana_program::{
    //     account_info::IntoAccountInfo, program_error::ProgramError, pubkey::Pubkey,
    // };
    // use solana_sdk::account::Account;
    // use spl_memo::processor::process_instruction;

    // #[test]
    // fn test_utf8_memo() {
    //     let program_id = Pubkey::new(&[0; 32]);
    //
    //     let string = b"letters and such";
    //     assert_eq!(Ok(()), process_instruction(&program_id, &[], string));
    //
    //     let emoji = "🐆".as_bytes();
    //     let bytes = [0xF0, 0x9F, 0x90, 0x86];
    //     assert_eq!(emoji, bytes);
    //     assert_eq!(Ok(()), process_instruction(&program_id, &[], &emoji));
    //
    //     let mut bad_utf8 = bytes;
    //     bad_utf8[3] = 0xFF; // Invalid UTF-8 byte
    //     assert_eq!(
    //         Err(ProgramError::InvalidInstructionData),
    //         process_instruction(&program_id, &[], &bad_utf8)
    //     );
    // }
    */

    pub fn initialize_vault(ctx: Context<InitializeVault>, token_type: Pubkey, vault_bump: u8) -> Result<()>
    {
        {
            let cpi_context = CpiContext::new(
                ctx.accounts.associated_token.to_account_info(),
                Create {
                    payer: ctx.accounts.payer.to_account_info(),
                    associated_token: ctx.accounts.vault_pool_skt_account.to_account_info(),
                    authority: ctx.accounts.vault_pool.to_account_info(),
                    mint: ctx.accounts.skt_mint.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                },
            );
            create(cpi_context)?;
        }

        let vault = &mut ctx.accounts.vault;
        vault.token_type = token_type;
        vault.vault_bump = vault_bump;

        msg!("Vault: {:?}", ctx.accounts.vault_pool.key);
        msg!("Vault Owner: {:?}", ctx.accounts.vault_pool.owner);
        msg!("System ID: {:?}", &System::id());

        Ok(())
    }

    pub fn withdraw_vault(ctx: Context<WithdrawVault>) -> Result<()>
    {
        let vault = &ctx.accounts.vault;
        let vault_address = vault.key().clone();

        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info().clone(),
            token::Transfer
            {
                from: ctx.accounts.vault_pool_skt_account.to_account_info().clone(),
                to: ctx.accounts.claimer_skt_account.to_account_info().clone(),
                authority: ctx.accounts.vault_pool.to_account_info().clone(),
            }
        );

        let seeds = [
            VAULT_SKT_SEED_PREFIX.as_bytes(),
            vault_address.as_ref(),
            &[vault.vault_bump],
        ];
        token::transfer(cpi_context.with_signer(&[&seeds[..]]), 3)?;

        Ok(())
    }

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
                    &anchor_raffle_ticket::id(),
                    &mut TransferSPLToken
                    {
                        sender: ctx.accounts.payer.clone(),
                        sender_tokens: ctx.accounts.sender_tokens.clone(),
                        recipient_tokens: ctx.accounts.recipient_tokens.clone(),
                        token_program: ctx.accounts.token_program.clone()
                    },
                    &[],
                    ctx.bumps.clone())
        )?;

        /* Option B: */
        {
        // token::transfer(
        //     CpiContext::new(
        //         ctx.accounts.token_program.to_account_info(),
        //         TransferSPL {
        //             from: ctx.accounts.sender_tokens.to_account_info(),
        //             to: ctx.accounts.recipient_tokens.to_account_info(),
        //             authority: ctx.accounts.payer.to_account_info(),
        //         },
        //     ),
        //     1,
        // )?;
        }

        msg!("Program initialized successfully.");
        msg!("Total Tickets: {:?}", raffle.total_tickets);
        msg!("Sold Tickets: {:?}", raffle.sold_tickets);
        msg!("Price Per Ticket: {} {}", raffle.price_per_ticket, raffle.price_per_ticket as f64 / LAMPORTS_PER_SOL as f64);
        msg!("Token SPL Address: {:?}", raffle.token_spl_address);
        msg!("New Raffle Account: {}", ctx.accounts.raffle.to_account_info().key());

        Ok(())
    }

    fn add_buyers(raffle: Raffle, buyer: Pubkey, amount: u32) {
        
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

        raffle.sold_tickets = raffle.sold_tickets.checked_add(amount).unwrap();

        emit!(BuyEvent
            {
                buyer: *ctx.accounts.buyer.to_account_info().key,
                amount: amount,
                sold_tickets: raffle.sold_tickets,
                total_tickets: raffle.total_tickets,
                remaining_tickets: raffle.total_tickets - raffle.sold_tickets
            });


        let remaining_tickets = raffle.total_tickets - raffle.sold_tickets;

        let index = raffle.buyers.iter().position(|x| x.key == ctx.accounts.buyer.key());
        if let Some(index) = index {
            let item = &mut raffle.buyers[index];
            item.tickets = item.tickets.checked_add(amount).unwrap();
        } else {
            let item = Buyer {
                key: ctx.accounts.buyer.key(),
                tickets: amount,
            };
            raffle.buyers.push(item);
        }

        msg!("Buyer: {:?}", *ctx.accounts.buyer.to_account_info().key);
        msg!("Total Tickets: {:?} | Sold {:?} | Remaining: {:?} | Price {:?} ({})", raffle.total_tickets, raffle.sold_tickets, remaining_tickets, raffle.price_per_ticket, raffle.price_per_ticket as f64 / LAMPORTS_PER_SOL as f64);
        msg!("Buy Amount: {:?} | Total Cost: {:?} ({})", amount, transaction_price, transaction_price as f64 / LAMPORTS_PER_SOL as f64);

        Ok(())
    }

    pub fn buy_ticket_spl(ctx: Context<BuyTicketSPL>, amount: u32, _ticket_price: u64, _token_spl_address: Pubkey) -> Result<()>
    {
        let raffle = &mut ctx.accounts.raffle;
        let transaction_price = raffle.price_per_ticket * amount as u64;

        //let dustPubKey = Pubkey::from_str("DUSTawucrTsGU8hcqRdHDCbuYhCPADMLM2VcCb8VnFnQ").unwrap(); // testing purposes

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

        raffle.sold_tickets = raffle.sold_tickets.checked_add(amount).unwrap();

        emit!(BuyEvent
            {
                buyer: *ctx.accounts.sender.to_account_info().key,
                amount: amount,
                sold_tickets: raffle.sold_tickets,
                total_tickets: raffle.total_tickets,
                remaining_tickets: raffle.total_tickets - raffle.sold_tickets
            });


        let remaining_tickets = raffle.total_tickets - raffle.sold_tickets;

        let index = raffle.buyers.iter().position(|x| x.key == ctx.accounts.sender.key());
        if let Some(index) = index {
            let item = &mut raffle.buyers[index];
            item.tickets = item.tickets.checked_add(amount).unwrap();
        } else {
            let item = Buyer {
                key: ctx.accounts.sender.key(),
                tickets: amount,
            };
            raffle.buyers.push(item);
        }

        msg!("Buyer: {:?}", *ctx.accounts.sender.to_account_info().key);
        msg!("Total Tickets: {:?} | Sold {:?} | Remaining: {:?} | Price {:?} ({})", raffle.total_tickets, raffle.sold_tickets, remaining_tickets, raffle.price_per_ticket, raffle.price_per_ticket as f64 / LAMPORTS_PER_SOL as f64);
        msg!("Buy Amount: {:?} | Total Cost: {:?} ({})", amount, transaction_price, transaction_price as f64 / LAMPORTS_PER_SOL as f64);

        Ok(())
    }

    pub fn transfer_spl_token(ctx: Context<TransferSPLToken>) -> Result<()>
    {
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                TransferSPL {
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
}