use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::state::{NftVault};
use crate::constants::*;

#[derive(Accounts)]
pub struct InitializeNftVault<'info> 
{
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(zero)]
    pub nft_vault: Account<'info, NftVault>,

    /// CHECK:
    #[account(
        init, 
        seeds = [NFT_VAULT_POOL_SEED.as_bytes(), nft_vault.key().as_ref()], 
        bump,
        payer = authority, 
        space = 0, 
    )]
    pub nft_vault_pool: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetMintPrice<'info>
{
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut, has_one = authority)]
    pub nft_vault: Account<'info, NftVault>
}

#[derive(Accounts)]
pub struct MintFromVault<'info>
{
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(mut)]
    pub nft_vault: Account<'info, NftVault>,

    /// CHECK:
    #[account(mut, seeds = [NFT_VAULT_POOL_SEED.as_bytes(), nft_vault.key().as_ref()], bump = nft_vault.pool_bump)]
    pub nft_vault_pool: AccountInfo<'info>,

    /// CHECK:
    pub nft_mint: AccountInfo<'info>,

    #[account(mut, constraint = vault_pool_ata.mint.key() == nft_mint.key())]
    pub vault_pool_ata: Account<'info, TokenAccount>,

    #[account(mut, constraint = vault_pool_ata.mint.key() == nft_mint.key())]
    pub buyer_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}