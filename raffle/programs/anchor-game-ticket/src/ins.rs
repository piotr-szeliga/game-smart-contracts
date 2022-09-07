use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::state::{Raffle, Vault, ErrorCode};

pub const VAULT_SKT_SEED_PREFIX: &str = "skt_pool";

#[derive(Accounts)]
pub struct Memo<'info> {
    /// CHECK:
    pub memo: AccountInfo<'info>
}

#[derive(Accounts)]
#[instruction(vault_bump: u8)]
pub struct InitializeVault<'info>
{
    #[account(mut)]
    pub payer: Signer<'info>,
    // vault
    #[account(init, payer = payer, space = Vault::LEN + 8)]
    pub vault: Account<'info, Vault>,
    // vault pool pda account ($skt token account)

    /// CHECK:
    #[account(seeds = [VAULT_SKT_SEED_PREFIX.as_bytes(), vault.key().as_ref()], bump = vault_bump)]
    pub vault_pool: AccountInfo<'info>,
    // vault pool $skt token account owned by vault
    #[account(mut)]
    pub vault_pool_skt_account: Account<'info, TokenAccount>,
    // $skt mint
    pub skt_mint: Account<'info, Mint>,
    pub rent: Sysvar<'info, Rent>,

    #[account(address = anchor_spl::associated_token::ID)]
    pub associated_token: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawVault<'info>
{
    // claimer authority
    #[account(mut)]
    pub claimer: Signer<'info>,
    // claimer skt account
    #[account(mut)]
    pub claimer_skt_account: Account<'info, TokenAccount>,
    // skt mint
    #[account(mut)]
    pub skt_mint: Account<'info, Mint>,
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    // skt pool account
    /// CHECK:
    #[account(mut, seeds = [VAULT_SKT_SEED_PREFIX.as_bytes(), vault.key().as_ref()], bump = vault.vault_bump)]
    pub vault_pool: AccountInfo<'info>,
    // vault pool skt token account
    #[account(mut)]
    pub vault_pool_skt_account: Account<'info, TokenAccount>,
    // associated token program
    #[account(address = anchor_spl::associated_token::ID)]
    pub associated_token_program: Program<'info, AssociatedToken>,
    // rent
    pub rent: Sysvar<'info, Rent>,
    // token program
    pub token_program: Program<'info, Token>,
    // system program
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Initialize<'info>
{
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init, payer = payer, space = Raffle::SPACE + 8)]
    pub raffle: Account<'info, Raffle>,
    #[account(address = anchor_lang::system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub sender_tokens: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_tokens: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(amount: u32, ticket_price: u64, token_spl_address: Pubkey)]
pub struct BuyTicketSOL<'info> // For SOL Transfers
{
    #[account(mut)]
    pub buyer: Signer<'info>,
    /// CHECK:
    #[account(mut)]
    pub recipient: AccountInfo<'info>,
    #[account(mut, constraint = amount + raffle.sold_tickets <= raffle.total_tickets @ ErrorCode::NoTicketsLeft,
    constraint = ticket_price == raffle.price_per_ticket @ ErrorCode::RafflePriceMismatched,
    constraint = token_spl_address == raffle.token_spl_address @ ErrorCode::RaffleTokenSPLAddressMismatched)]
    pub raffle: Account<'info, Raffle>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(amount: u32, ticket_price: u64, token_spl_address: Pubkey)]
pub struct BuyTicketSPL<'info> // For SPL-Token Transfer
{
    pub sender: Signer<'info>,
    #[account(mut)]
    pub sender_tokens: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_tokens: Account<'info, TokenAccount>,
    #[account(mut,
    constraint = amount + raffle.sold_tickets <= raffle.total_tickets @ ErrorCode::NoTicketsLeft,
    constraint = ticket_price == raffle.price_per_ticket @ ErrorCode::RafflePriceMismatched,
    constraint = token_spl_address == raffle.token_spl_address @ ErrorCode::RaffleTokenSPLAddressMismatched)]
    pub raffle: Account<'info, Raffle>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct TransferSPLToken<'info> // For SPL-Token Transfer
{
    pub sender: Signer<'info>,
    #[account(mut)]
    pub sender_tokens: Account<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_tokens: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Convert<'info> {
    // claimer authority
    #[account(mut)]
    pub claimer: Signer<'info>,
    // claimer skt account
    #[account(mut)]
    pub claimer_skt_account: Account<'info, TokenAccount>,
    // skt mint
    #[account(mut)]
    pub skt_mint: Account<'info, Mint>,
    /// CHECK:
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    // skt pool account
    /// CHECK:
    #[account(mut, seeds = [VAULT_SKT_SEED_PREFIX.as_bytes(), vault.key().as_ref()], bump = vault.vault_bump)]
    pub vault_pool: AccountInfo<'info>,
    // vault pool skt token account
    #[account(mut)]
    pub vault_pool_skt_account: Account<'info, TokenAccount>,
    // associated token program 
    #[account(address = anchor_spl::associated_token::ID)]
    pub  associated_token_program: Program<'info, AssociatedToken>,
    // rent
    pub rent: Sysvar<'info, Rent>,
    // token program
    pub token_program: Program<'info, Token>,
    // system program
    pub system_program: Program<'info, System>,
}