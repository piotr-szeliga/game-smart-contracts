use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateGift<'info> 
{
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(mut)]
    pub gift: Account<'info, Gift>,

    #[account(mut)]
    pub spl_token_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = spl_token_mint,
        associated_token::authority = creator_token_ata,
    )]
    pub creator_token_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = spl_token_mint,
        associated_token::authority = gift,
    )]
    pub gift_token_ata: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = creator,
        
    )]

    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = gift,
    )]
    pub gift_nft_ata: Account<'info, TokenAccount>,

}