#![feature(core_intrinsics)]
use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer as TransferProgramSOL};
use anchor_spl::token::{self, Token, TokenAccount, Transfer as TransferSPL};

declare_id!("AGyQHJtznL3WiqWzsV31Rxpvvk4qwZHnaUVn9LPdnjZj");

pub const LAMPORTS_PER_SOL: u64 = 1000000000;

#[program]
pub mod anchor_raffle_ticket {
    use anchor_lang::system_program;
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, token_type: Pubkey, price: u64, amount: u32) -> Result<()> {
        let raffle = &mut ctx.accounts.raffle;
        raffle.price_per_ticket = price;
        raffle.total_tickets = amount;
        raffle.sold_tickets = 0;
        raffle.token_type = token_type;

        msg!("Program initialized successfully.");
        msg!("Total Tickets: {:?}", raffle.total_tickets);
        msg!("Sold Tickets: {:?}", raffle.sold_tickets);
        msg!("Price Per Ticket: {} {}", raffle.price_per_ticket, raffle.price_per_ticket as f64 / LAMPORTS_PER_SOL as f64);
        msg!("Token Type: {:?}", raffle.token_type);
        msg!("New Raffle Account: {}", ctx.accounts.raffle.to_account_info().key());

        Ok(())
    }

    pub fn buy_ticket(ctx: Context<BuyTicket>, amount: u32) -> Result<()>
    {
        let raffle = &mut ctx.accounts.raffle;
        let transaction_price = raffle.price_per_ticket * amount as u64;

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

        // transfer via SPL-Token

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
        msg!("Total Tickets: {:?} | Sold {:?} | Remaining: {:?} | Price {:?}", raffle.total_tickets, raffle.sold_tickets, remaining_tickets, raffle.price_per_ticket);
        msg!("Buy Amount: {:?} | Total Cost: {:?}", amount, transaction_price);


        Ok(())
    }

    pub fn transfer_tokens(ctx: Context<TransferTokens>, amount: u64) -> Result<()>
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
            amount,
        )?;

        return Ok(());
    }
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct TransferTokens<'info> {
    pub sender: Signer<'info>,
    #[account(mut)]
    pub sender_tokens: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_tokens: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
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
#[instruction(amount: u32, xx: bool)]
pub struct BuyTicket<'info> {
    // buyer account
    #[account(mut)]
    buyer: Signer<'info>,
    // recipient
    /// CHECK:
    #[account(mut)]
    recipient: AccountInfo<'info>,
    // raffle
    #[account(mut, constraint = amount + raffle.sold_tickets <= raffle.total_tickets @ ErrorCode::NoTicketsLeft)]
    raffle: Account<'info, Raffle>,
    // system program
    system_program: Program<'info, System>,
}

#[account]
pub struct Raffle {
    pub total_tickets: u32,
    pub sold_tickets: u32,
    pub price_per_ticket: u64,
    pub token_type: Pubkey,
    //pub is_sol: bool
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
}