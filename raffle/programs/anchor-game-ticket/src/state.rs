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
pub struct Global {
    pub authority: Pubkey,
    pub authorized_admins: Vec<Pubkey>,
}

impl Global {
    pub const LEN: usize = 1 + 32 * 10;
}

#[account]
pub struct Raffle
{
    pub pool_bump: u8,
    pub total_tickets: u32,
    pub sold_tickets: u32,
    pub price_per_ticket: u64,
    pub token_spl_address: Pubkey,
    pub owner: Pubkey,
    pub store_buyers: bool,
    pub buyers: Vec<Buyer>,
}

impl Raffle
{
    pub const SPACE: usize =  std::mem::size_of::<Raffle>();
}

#[derive(Debug, AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Copy)]
pub struct Buyer {
    pub key: Pubkey,
    pub tickets: u32,
}

#[account]
pub struct NftVault {
    pub authority: Pubkey,
    pub pool_bump: u8,
    pub mint_price: u64,
    pub total_supply: u32,
    pub sold_mints: Vec<Pubkey>,
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
    #[msg("Already authorized admin")] // 0x1775
    AlreadyAuthorizedAdmin,
    #[msg("Not authorized admin")] // 0x1776
    NotAuthorizedAdmin,
    #[msg("Cannot withdraw morethan 10,000")] // 0x1777
    ExceedMaxWithdrawAmount,
    #[msg("Already Minted")] // 0x1778
    AlreadyMinted,
    #[msg("Not Enough Sol")] // 0x1779
    NotEnoughSol
}