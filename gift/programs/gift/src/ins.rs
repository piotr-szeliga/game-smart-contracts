use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, TokenAccount, Token},
    associated_token::AssociatedToken,
};

use crate::state::*;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct InitializeGlobal<'info>
{
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = Global::LEN + 8,
        seeds = [
            b"global".as_ref(),
            name.as_ref(),
            authority.key().as_ref()
        ],
        bump
    )]
    pub global: Account<'info, Global>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateGlobal<'info>
{
    #[account(mut, address = global.authority)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [
            b"global".as_ref(),
            global.name.as_ref(),
            authority.key().as_ref()
        ],
        bump = global.bump
    )]
    pub global: Account<'info, Global>,
}


#[derive(Accounts)]
pub struct CreateGift<'info> 
{
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(mut)]
    pub target: SystemAccount<'info>,

    #[account(mut)]
    pub nft_mint: Box<Account<'info, Mint>>,

    /// CHECK:
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,

    #[account(
        init,
        payer = creator,
        seeds = [
            b"gift".as_ref(),
            nft_mint.key().as_ref(),
        ],
        space = Gift::LEN + 8,
        bump
    )]
    pub gift: Box<Account<'info, Gift>>,

    #[account(
        seeds = [
            b"global".as_ref(),
            global.name.as_ref(),
            global.authority.key().as_ref()
        ],
        bump = global.bump
    )]
    pub global: Box<Account<'info, Global>>,

    #[account(mut)]
    pub spl_token_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = spl_token_mint,
        associated_token::authority = creator,
    )]
    pub creator_token_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = creator,
        associated_token::mint = spl_token_mint,
        associated_token::authority = gift,
    )]
    pub gift_token_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = target,
    )]
    pub target_nft_ata: Box<Account<'info, TokenAccount>>,

    /// CHECK:
    pub token_metadata_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Redeem<'info>
{
    #[account(mut)]
    pub target: Signer<'info>,

    #[account(mut, address = gift.nft_mint)]
    pub nft_mint: Box<Account<'info, Mint>>,

    #[account(  
        mut,    
        associated_token::mint = nft_mint,  
        associated_token::authority = target    
    )]  
    pub target_nft_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [
            b"gift".as_ref(),
            nft_mint.key().as_ref(),
        ],
        bump = gift.bump
    )]
    pub gift: Box<Account<'info, Gift>>,

    #[account(
        associated_token::mint = gift.gate_token_mint,
        associated_token::authority = target,
    )]
    pub gate_token_ata: Box<Account<'info, TokenAccount>>,

    #[account(mut, address = gift.spl_token_mint)]
    pub spl_token_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = spl_token_mint,
        associated_token::authority = gift,
    )]
    pub gift_token_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = target,
        associated_token::mint = spl_token_mint,
        associated_token::authority = target,
    )]
    pub target_token_ata: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

 
    pub rent: Sysvar<'info, Rent>,
}