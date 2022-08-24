use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

declare_id!("BvXkNSWxqAszoJyRU6eQJB6cYAWsk7JvsLt7JtHhks2b");

pub const PRICE_PER_TICKET: u64 = 150_000_000;

#[program]
pub mod anchor_game_ticket {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, amount: u32) -> Result<()> {
        let game = &mut ctx.accounts.game;
        game.total_tickets = amount;
        game.sold_tickets = 0;
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

        transfer(cpi_context, PRICE_PER_TICKET * amount as u64)?;

        let game = &mut ctx.accounts.game;
        game.sold_tickets = game.sold_tickets.checked_add(amount).unwrap();
        
        emit!(BuyEvent {
            buyer: *ctx.accounts.buyer.to_account_info().key,
            amount: amount
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // payer
    #[account(mut)]
    payer: Signer<'info>,
    // game
    #[account(init, payer = payer, space = Game::LEN + 8)]
    game: Account<'info, Game>,
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
    // game
    #[account(mut, constraint = amount + game.sold_tickets <= game.total_tickets)]
    game: Account<'info, Game>,
    // system program
    system_program: Program<'info, System>,
}

#[account]
pub struct Game {
    pub total_tickets: u32,
    pub sold_tickets: u32,
}

impl Game {
    pub const LEN: usize = 4 + 4;
}

#[event]
pub struct BuyEvent {
    pub buyer: Pubkey,
    pub amount: u32,
}