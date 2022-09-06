use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::associated_token::{Create, create};
use anchor_spl::token;
use crate::ins::*;

pub fn initialize_vault(ctx: Context<InitializeVault>, token_type: Pubkey, vault_bump: u8) -> Result<()>
{
    {
        let cpi_context = CpiContext::new(
            ctx.accounts.associated_token.to_account_info(),
            Create {
                payer: ctx.accounts.payer.to_account_info(),
                associated_token: ctx.accounts.vault_pool_skt_account.to_account_info(),
                authority: ctx.accounts.vault_pool.to_account_info(),
                mint: ctx.accounts.skt_mint.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            },
        );
        create(cpi_context)?;
    }

    let vault = &mut ctx.accounts.vault;
    vault.token_type = token_type;
    vault.vault_bump = vault_bump;

    msg!("Vault: {:?}", ctx.accounts.vault_pool.key);
    msg!("Vault Owner: {:?}", ctx.accounts.vault_pool.owner);
    msg!("System ID: {:?}", &System::id());

    // let account_info = vec![
    //     ctx.accounts.buyer_authority.to_account_info()
    // ];

    // invoke(
    //     &build_memo("Hello world".as_bytes(), &[&ctx.accounts.buyer_authority.key()]),
    //     account_info.as_slice()
    // )?;

    


    Ok(())
}

pub fn withdraw_vault(ctx: Context<WithdrawVault>) -> Result<()>
{
    let vault = &ctx.accounts.vault;
    let vault_address = vault.key().clone();

    let cpi_context = CpiContext::new(
        ctx.accounts.token_program.to_account_info().clone(),
        token::Transfer
        {
            from: ctx.accounts.vault_pool_skt_account.to_account_info().clone(),
            to: ctx.accounts.claimer_skt_account.to_account_info().clone(),
            authority: ctx.accounts.vault_pool.to_account_info().clone(),
        }
    );

    let seeds = [
        VAULT_SKT_SEED_PREFIX.as_bytes(),
        vault_address.as_ref(),
        &[vault.vault_bump],
    ];
    token::transfer(cpi_context.with_signer(&[&seeds[..]]), 3)?;

    Ok(())
}

pub fn convert_skt_sol(ctx: Context<Convert>, exchange_option: u8, is_holder: bool) -> Result<()> {
    let vault = &ctx.accounts.vault;
    let vault_address = vault.key().clone();
    
    let sol_amount = match is_holder {
        false => match exchange_option {
            0 => 400_000_000,
            1 => 600_000_000,
            2 => 1_000_000_000,
            _ => 1_600_000_000
        },
        true => match exchange_option {
            0 => 300_000_000,
            1 => 500_000_000,
            2 => 800_000_000,
            _ => 1_300_000_000
        }
    };

    let skt_amount = match exchange_option {
        0 => 70_000_000_000,
        1 => 140_000_000_000,
        2 => 320_000_000_000,
        _ => 500_000_000_000
    };

    {
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info().clone(),
            system_program::Transfer {
                from: ctx.accounts.claimer.to_account_info().clone(),
                to: ctx.accounts.vault.to_account_info().clone(),
            },
        );

        system_program::transfer(cpi_context, sol_amount)?;
    }
   
    {
        {
            let cpi_context = CpiContext::new(
                ctx.accounts.associated_token_program.to_account_info().clone(),
                Create {
                    payer: ctx.accounts.claimer.to_account_info().clone(),
                    associated_token: ctx.accounts.claimer_skt_account.to_account_info().clone(),
                    authority: ctx.accounts.claimer.to_account_info().clone(),
                    mint: ctx.accounts.skt_mint.to_account_info().clone(),
                    rent: ctx.accounts.rent.to_account_info().clone(),
                    token_program: ctx.accounts.token_program.to_account_info().clone(),
                    system_program: ctx.accounts.system_program.to_account_info().clone(),
                }
            );
            create(cpi_context)?;
        }

        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info().clone(),
            anchor_spl::token::Transfer {
                from: ctx.accounts.vault_pool_skt_account.to_account_info().clone(),
                to: ctx.accounts.claimer_skt_account.to_account_info().clone(),
                authority: ctx.accounts.vault_pool.to_account_info().clone(),
            }
        );
        let seeds = [
            VAULT_SKT_SEED_PREFIX.as_bytes(),
            vault_address.as_ref(),
            &[vault.vault_bump],
        ];
        anchor_spl::token::transfer(cpi_context.with_signer(&[&seeds[..]]), skt_amount)?;
    }
    Ok(())
}