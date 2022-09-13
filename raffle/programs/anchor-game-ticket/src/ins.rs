use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::state::{Raffle, Vault, Global, ErrorCode};

pub const VAULT_SKT_SEED_PREFIX: &str = "skt_pool";
pub const RAFFLE_POOL_SEED_PREFIX: &str = "raffle_pool";

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
    #[account(seeds = [VAULT_SKT_SEED_PREFIX.as_bytes(), vault.key().as_ref()], bump = vault_bump)]
    pub vault_pool: SystemAccount<'info>,
    // vault pool $skt token account owned by vault
    /// CHECK:
    #[account(mut)]
    pub vault_pool_skt_account: UncheckedAccount<'info>,
    // $skt mint
    pub skt_mint: Account<'info, Mint>,
    pub rent: Sysvar<'info, Rent>,

    /// CHECK:
    pub memo: AccountInfo<'info>,
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
    // #[account(zero)]
    // pub raffle: AccountLoader<'info, Raffle>,
    #[account(init, payer = payer, space = Raffle::SPACE + 8)]
    pub raffle: Account<'info, Raffle>,

    #[account(mut)]
    pub sender_tokens: Account<'info, TokenAccount>,

    #[account(mut)]
    pub recipient_tokens: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,

    #[account(address = anchor_lang::system_program::ID)]
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(pool_bump: u8)]
pub struct InitializeWithPDA<'info>
{
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(init, payer = payer, space = Raffle::SPACE + 8)]
    pub raffle: Account<'info, Raffle>,

    #[account(mut)]
    pub sender_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub raffle_pool_ata: Account<'info, TokenAccount>,

    // token program
    pub token_program: Program<'info, Token>,

    #[account(address = anchor_lang::system_program::ID)]
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeGlobal<'info>
{
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(init, payer = payer, space = Global::LEN + 8)]
    pub global: Account<'info, Global>,

    pub admin: SystemAccount<'info>,
    
    #[account(address = anchor_lang::system_program::ID)]
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ControlAdmins<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        has_one = authority,
    )]
    pub global: Account<'info, Global>,
    /// CHECK:
    pub admin: AccountInfo<'info>,
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
    #[account(
        mut, 
        // constraint = amount + raffle.load()?.sold_tickets <= raffle.load()?.total_tickets @ ErrorCode::NoTicketsLeft,
        // constraint = ticket_price == raffle.load()?.price_per_ticket @ ErrorCode::RafflePriceMismatched,
        // constraint = token_spl_address == raffle.load()?.token_spl_address @ ErrorCode::RaffleTokenSPLAddressMismatched
        constraint = amount + raffle.sold_tickets <= raffle.total_tickets @ ErrorCode::NoTicketsLeft,
        constraint = ticket_price == raffle.price_per_ticket @ ErrorCode::RafflePriceMismatched,
        constraint = token_spl_address == raffle.token_spl_address @ ErrorCode::RaffleTokenSPLAddressMismatched
    )]
    // pub raffle: AccountLoader<'info, Raffle>,
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
    #[account(
        mut,
        // constraint = amount + raffle.load()?.sold_tickets <= raffle.load()?.total_tickets @ ErrorCode::NoTicketsLeft,
        // constraint = ticket_price == raffle.load()?.price_per_ticket @ ErrorCode::RafflePriceMismatched,
        // constraint = token_spl_address == raffle.load()?.token_spl_address @ ErrorCode::RaffleTokenSPLAddressMismatched
        constraint = amount + raffle.sold_tickets <= raffle.total_tickets @ ErrorCode::NoTicketsLeft,
        constraint = ticket_price == raffle.price_per_ticket @ ErrorCode::RafflePriceMismatched,
        constraint = token_spl_address == raffle.token_spl_address @ ErrorCode::RaffleTokenSPLAddressMismatched
    )]
    // pub raffle: AccountLoader<'info, Raffle>,
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
    /// CHECK:
    #[account(mut)]
    pub claimer_skt_account: AccountInfo<'info>,
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