#![feature(core_intrinsics)]
use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer as TransferProgramSOL};
use anchor_spl::token::{self, Token, TokenAccount, Transfer as TransferSPL};
// use solana_sdk::{
//     signature::{Keypair, Signer},
//     transaction::{Transaction, TransactionError},
// };
// use spl_memo::*;

declare_id!("AGyQHJtznL3WiqWzsV31Rxpvvk4qwZHnaUVn9LPdnjZj");

pub const LAMPORTS_PER_SOL: u64 = 1000000000;

#[program]
pub mod anchor_raffle_ticket
{
    //use std::str::FromStr;
    use anchor_lang::system_program;
    use super::*;

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

    pub fn initialize(ctx: Context<Initialize>, token_type: Pubkey, ticket_price: u64, amount: u32) -> Result<()>
    {
        // let memo = "🐆".as_bytes();
        // let keypairs = vec![Keypair::new(), Keypair::new(), Keypair::new()];
        // let pubkeys: Vec<Pubkey> = keypairs.iter().map(|keypair| keypair.pubkey()).collect();
        // let mut transaction = Transaction::new_with_payer(&[build_memo(memo, &signer_key_refs)], Some(&payer.pubkey()));
        // let mut signers = vec![&payer];
        // for keypair in keypairs.iter() {
        //     signers.push(keypair);
        // }
        // transaction.sign(&signers, recent_blockhash);

        let raffle = &mut ctx.accounts.raffle;
        raffle.token_type = token_type;
        raffle.price_per_ticket = ticket_price;
        raffle.total_tickets = amount;
        raffle.sold_tickets = 0;

        msg!("Program initialized successfully.");
        msg!("Total Tickets: {:?}", raffle.total_tickets);
        msg!("Sold Tickets: {:?}", raffle.sold_tickets);
        msg!("Price Per Ticket: {} {}", raffle.price_per_ticket, raffle.price_per_ticket as f64 / LAMPORTS_PER_SOL as f64);
        msg!("Token Type: {:?}", raffle.token_type);
        msg!("New Raffle Account: {}", ctx.accounts.raffle.to_account_info().key());

        Ok(())
    }

    pub fn buy_ticket_sol(ctx: Context<BuyTicketSOL>, amount: u32, _ticket_price: u64) -> Result<()>
    {
        let raffle = &mut ctx.accounts.raffle;
        let transaction_price = raffle.price_per_ticket * amount as u64;

        //if raffle.token_type.key().to_string() == "11111111111111111111111111111111" // Paying with SOL
        msg!("SOL Transfer: {:?}", raffle.token_type.key());

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
        msg!("Buyer: {:?}", *ctx.accounts.buyer.to_account_info().key);
        msg!("Total Tickets: {:?} | Sold {:?} | Remaining: {:?} | Price {:?} ({})", raffle.total_tickets, raffle.sold_tickets, remaining_tickets, raffle.price_per_ticket, raffle.price_per_ticket as f64 / LAMPORTS_PER_SOL as f64);
        msg!("Buy Amount: {:?} | Total Cost: {:?} ({})", amount, transaction_price, transaction_price as f64 / LAMPORTS_PER_SOL as f64);

        Ok(())
    }

    pub fn buy_ticket_spl(ctx: Context<BuyTicketSPL>, amount: u32, _ticket_price: u64) -> Result<()>
    {
        let raffle = &mut ctx.accounts.raffle;
        let transaction_price = raffle.price_per_ticket * amount as u64;

        //let dustPubKey = Pubkey::from_str("DUSTawucrTsGU8hcqRdHDCbuYhCPADMLM2VcCb8VnFnQ").unwrap(); // testing purposes

        msg!("SPL-Token Transfer: {:?}", raffle.token_type.key());

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

        msg!("Token Type: {:?}", raffle.token_type.key());

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
        msg!("Buyer: {:?}", *ctx.accounts.sender.to_account_info().key);
        msg!("Total Tickets: {:?} | Sold {:?} | Remaining: {:?} | Price {:?} ({})", raffle.total_tickets, raffle.sold_tickets, remaining_tickets, raffle.price_per_ticket, raffle.price_per_ticket as f64 / LAMPORTS_PER_SOL as f64);
        msg!("Buy Amount: {:?} | Total Cost: {:?} ({})", amount, transaction_price, transaction_price as f64 / LAMPORTS_PER_SOL as f64);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info>
{
    // payer
    #[account(mut)]
    payer: Signer<'info>,
    // raffle
    #[account(init, payer = payer, space = Raffle::SPACE + 8)]
    raffle: Account<'info, Raffle>,
    // system program
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(amount: u32, ticket_price: u64)]
pub struct BuyTicketSOL<'info> // For SOL Transfers
{
    // buyer account
    #[account(mut)]
    buyer: Signer<'info>,
    // recipient
    /// CHECK:
    #[account(mut)]
    recipient: AccountInfo<'info>,
    // raffle
    #[account(mut, constraint = amount + raffle.sold_tickets <= raffle.total_tickets @ ErrorCode::NoTicketsLeft, constraint = ticket_price == raffle.price_per_ticket @ ErrorCode::RafflePriceMismatched)]
    raffle: Account<'info, Raffle>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(amount: u32, ticket_price: u64)]
pub struct BuyTicketSPL<'info> // For SPL-Token Transfer
{
    sender: Signer<'info>,
    #[account(mut)]
    sender_tokens: Account<'info, TokenAccount>,
    #[account(mut)]
    recipient_tokens: Account<'info, TokenAccount>,
    #[account(mut,
    constraint = amount + raffle.sold_tickets <= raffle.total_tickets @ ErrorCode::NoTicketsLeft,
    constraint = ticket_price == raffle.price_per_ticket @ ErrorCode::RafflePriceMismatched)]
    raffle: Account<'info, Raffle>,
    token_program: Program<'info, Token>,
}

#[account]
pub struct Raffle {
    pub total_tickets: u32,
    pub sold_tickets: u32,
    pub price_per_ticket: u64,
    pub token_type: Pubkey,
}

impl Raffle {
    pub const SPACE: usize = std::mem::size_of::<Raffle>();
}

#[event]
pub struct BuyEvent {
    pub buyer: Pubkey,
    pub amount: u32,
    pub sold_tickets: u32,
    pub total_tickets: u32,
    pub remaining_tickets: u32
}

#[error_code]
pub enum ErrorCode {
    #[msg("No more tickets left for purchase.")]
    NoTicketsLeft,
    #[msg("Raffle price mismatched.")]
    RafflePriceMismatched,
}