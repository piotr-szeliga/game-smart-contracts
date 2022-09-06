use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Transfer};
use anchor_lang::solana_program::program::invoke;
use spl_memo::build_memo;

use crate::ins::*;

pub fn transfer_spl_token(ctx: Context<TransferSPLToken>) -> Result<()>
{
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.sender_tokens.to_account_info(),
                to: ctx.accounts.recipient_tokens.to_account_info(),
                authority: ctx.accounts.sender.to_account_info(),
            },
        ),
        1,
    )?;

    msg!("Transfer {} Done!",  ctx.accounts.recipient_tokens.mint);
    msg!("System ID {}!",  &System::id());

    Ok(())
}

pub fn memo(ctx: Context<Memo>) -> Result<()> {
    let account_info = vec![
        ctx.accounts.memo.to_account_info()
    ];

    invoke(
        &build_memo("Hello world".as_bytes(), &[]),
        account_info.as_slice()
    )?;

    Ok(())
}