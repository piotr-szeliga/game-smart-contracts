use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
//use anchor_spl::{self, associated_token::{AssociatedToken}, token::{self, Mint, TokenAccount, Token}};

declare_id!("AGyQHJtznL3WiqWzsV31Rxpvvk4qwZHnaUVn9LPdnjZj");

pub const PRICE_PER_TICKET: u64 = 150_000_000;
pub const LAMPORTS_PER_SOL: u64 = 1000000000;

#[program]
pub mod anchor_raffle_ticket {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, price: u64, amount: u32) -> Result<()> {
        let raffle = &mut ctx.accounts.raffle;
        raffle.price_per_ticket = price;
        raffle.total_tickets = amount;
        raffle.sold_tickets = 0;

        msg!("Program initialized successfully.");
        msg!("Total Tickets: {:?}", raffle.total_tickets);
        msg!("Sold Tickets: {:?}", raffle.sold_tickets);
        msg!("Price Per Ticket: {} {}", raffle.price_per_ticket, raffle.price_per_ticket as f64 / LAMPORTS_PER_SOL as f64);

        Ok(())
    }

    pub fn buy_ticket(ctx: Context<BuyTicket>, amount: u32) -> Result<()> {
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.buyer.to_account_info().clone(),
                to: ctx.accounts.recipient.to_account_info(),
            },
        );

        let raffle = &mut ctx.accounts.raffle;

        //transfer(cpi_context, raffle.price_per_ticket * LAMPORTS_PER_SOL * amount as u64)?;
        transfer(cpi_context, PRICE_PER_TICKET * amount as u64)?;

        raffle.sold_tickets = raffle.sold_tickets.checked_add(amount).unwrap();

        emit!(BuyEvent
            {
                buyer: *ctx.accounts.buyer.to_account_info().key,
                amount: amount,
                sold_tickets: raffle.sold_tickets,
                total_tickets: raffle.total_tickets,
                remaining_tickets: raffle.total_tickets - raffle.sold_tickets
            });


        msg!("Buyer: {:?}", *ctx.accounts.buyer.to_account_info().key);
        msg!("Buy Amount: {:?}", amount);
        msg!("Total Tickets: {:?}", raffle.total_tickets);
        msg!("Sold Tickets: {:?}", raffle.sold_tickets);
        msg!("Remaining Tickets: {:?}", raffle.total_tickets - raffle.sold_tickets);

        Ok(())
    }

    // pub fn create_mint_and_vault(ctx: Context<Initialize>, _decimals: u8, amount: u64) -> Result<T> {
    //     msg!("create_mint_and_vault begin");
    //     let mint_to_ctx = token::MintTo {
    //         mint: ctx.accounts.mint.to_account_info(),
    //         to: ctx.accounts.vault.to_account_info(),
    //         authority: ctx.accounts.authority.to_account_info()
    //     };
    //     return token::mint_to(CpiContext::new(ctx.accounts.token_program.to_account_info(), mint_to_ctx), amount);
    // }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // payer
    #[account(mut)]
    payer: Signer<'info>,
    // raffle
    #[account(init, payer = payer, space = Raffle::LEN + 8)]
    raffle: Account<'info, Raffle>,
    // system program
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(amount: u32)]
pub struct BuyTicket<'info> {
    // buyer account
    #[account(mut)]
    buyer: Signer<'info>,
    // recipient
    /// CHECK:
    #[account(mut)]
    recipient: AccountInfo<'info>,
    // raffle
    #[account(mut, constraint = amount + raffle.sold_tickets <= raffle.total_tickets)]
    raffle: Account<'info, Raffle>,
    // system program
    system_program: Program<'info, System>,
}

#[account]
pub struct Raffle {
    pub total_tickets: u32,
    pub sold_tickets: u32,
    pub price_per_ticket: u64,
}

impl Raffle {
    pub const LEN: usize = 4 + 4 + 8;
}

#[event]
pub struct BuyEvent {
    pub buyer: Pubkey,
    pub amount: u32,
    pub sold_tickets: u32,
    pub total_tickets: u32,
    pub remaining_tickets: u32
}