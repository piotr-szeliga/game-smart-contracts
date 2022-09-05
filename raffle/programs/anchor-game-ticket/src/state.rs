use anchor_lang::prelude::*;

#[account]
pub struct Vault
{
    pub token_type: Pubkey,
    pub vault_bump: u8,
}

impl Vault
{
    pub const LEN: usize = std::mem::size_of::<Vault>();
}

#[account]
pub struct Raffle
{
    pub total_tickets: u32,
    pub sold_tickets: u32,
    pub price_per_ticket: u64,
    pub token_spl_address: Pubkey,
    pub buyers: Vec<Buyer>,
}

impl Raffle
{
    pub const SPACE: usize = std::mem::size_of::<Raffle>();
}

#[derive(Debug, AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub struct Buyer {
    pub key: Pubkey,
    pub tickets: u32,
}


#[event]
pub struct BuyEvent
{
    pub buyer: Pubkey,
    pub amount: u32,
    pub sold_tickets: u32,
    pub total_tickets: u32,
    pub remaining_tickets: u32
}


#[error_code]
pub enum ErrorCode
{
    #[msg("No more tickets left for purchase.")] // 0x1770
    NoTicketsLeft,
    #[msg("Raffle price mismatched.")] // 0x1771
    RafflePriceMismatched,
    #[msg("Token Address mismatched.")] // 0x1772
    RaffleTokenSPLAddressMismatched,
    #[msg("Not Enough Tokens.")] // 0x1773
    NotEnoughTokens,
    #[msg("Custom Error.")] // 0x1774
    ErrorCustom,
}