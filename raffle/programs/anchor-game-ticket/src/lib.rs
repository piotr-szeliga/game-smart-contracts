#![feature(core_intrinsics)]

mod ins;
mod state;
mod raffle;
mod utils;
mod vault;
mod global;
mod constants;

use anchor_lang::prelude::*;

use ins::*;

declare_id!("3ug8uwLeeJjA8yz7jYEe33ntz7wD9x2SWXPxwC4zJkm5");


#[program]
pub mod anchor_raffle_ticket
{
    use super::*;

    
    pub fn memo(ctx: Context<Memo>) -> Result<()> {
        utils::memo(ctx, "Hello world")
    }

    pub fn transfer_spl_token(ctx: Context<TransferSPLToken>) -> Result<()> {
        utils::transfer_spl_token(ctx)
    }

    pub fn initialize_vault(ctx: Context<InitializeVault>, vault_bump: u8, token_type: Pubkey) -> Result<()>
    {
        vault::initialize_vault(ctx, token_type, vault_bump)
    }

    pub fn withdraw_vault(ctx: Context<WithdrawVault>, amount: u64) -> Result<()>
    {
        vault::withdraw_vault(ctx, amount)
    }

    pub fn convert_skt_sol(ctx: Context<Convert>, exchange_option: u8, is_holder: bool) -> Result<()> {
        vault::convert_skt_sol(ctx, exchange_option, is_holder)
    }

    pub fn initialize_global(ctx: Context<InitializeGlobal>) -> Result<()> {
        global::initialize_global(ctx)
    }

    pub fn authorize_admin(ctx: Context<ControlAdmins>) -> Result<()> {
        global::authorize_admin(ctx)
    }

    pub fn unauthorize_admin(ctx: Context<ControlAdmins>) -> Result<()> {
        global::unauthorize_admin(ctx)
    }

    pub fn initialize(ctx: Context<Initialize>, token_spl_address: Pubkey, ticket_price: u64, amount: u32, store_buyers: bool) -> Result<()>
    {
        raffle::initialize(ctx, token_spl_address, ticket_price, amount, store_buyers)
    }

    pub fn initialize_with_pda(ctx: Context<InitializeWithPDA>, pool_bump: u8, token_spl_address: Pubkey, ticket_price: u64, amount: u32, store_buyers: bool) -> Result<()> 
    {
        raffle::initialize_with_pda(ctx, pool_bump, token_spl_address, ticket_price, amount, store_buyers)
    }

    pub fn buy_ticket_sol(ctx: Context<BuyTicketSOL>, amount: u32, _ticket_price: u64, _token_spl_address: Pubkey) -> Result<()>
    {
        raffle::buy_ticket_sol(ctx, amount, _ticket_price, _token_spl_address)
    }

    pub fn buy_ticket_spl(ctx: Context<BuyTicketSPL>, amount: u32, _ticket_price: u64, _token_spl_address: Pubkey) -> Result<()>
    {
        raffle::buy_ticket_spl(ctx, amount, _ticket_price, _token_spl_address)
    }

    pub fn withdraw_from_pda(ctx: Context<WithdrawFromPDA>, amount: u64) -> Result<()>
    {
        raffle::withdraw_from_pda(ctx, amount)
    }
}