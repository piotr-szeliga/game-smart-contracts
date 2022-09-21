use anchor_lang::{
  prelude::*,
  system_program
};
use anchor_spl::token::{transfer, Transfer};
use crate::ins::*;
use crate::constants::*;
use crate::state::{ErrorCode};

pub fn initialize_nft_vault(ctx: Context<InitializeNftVault>, pool_bump: u8, mint_price: u64, total_supply: u32) -> Result<()>
{
  let nft_vault = &mut ctx.accounts.nft_vault;
  nft_vault.authority = ctx.accounts.authority.key();
  nft_vault.pool_bump = pool_bump;
  nft_vault.mint_price = mint_price;
  nft_vault.total_supply = total_supply;
  nft_vault.sold_mints = vec![];

  msg!("Mint Price: {:?}", mint_price);
  msg!("Total Supply: {:?}", total_supply);

  Ok(())
}

pub fn set_mint_price(ctx: Context<SetMintPrice>, mint_price: u64) -> Result<()>
{
  let nft_vault = &mut ctx.accounts.nft_vault;
  nft_vault.mint_price = mint_price;

  msg!("Mint Price: {:?}", mint_price);

  Ok(())
}

pub fn mint_from_vault(ctx: Context<MintFromVault>) -> Result<()>
{
  let nft_vault = &mut ctx.accounts.nft_vault;

  if nft_vault.sold_mints.iter().any(|x| x == &ctx.accounts.nft_mint.key()) {
    return Err(ErrorCode::AlreadyMinted.into());
  }

  if nft_vault.total_supply as usize == nft_vault.sold_mints.len()  {
    return Err(ErrorCode::NotEnoughTokens.into());
  }

  if ctx.accounts.buyer.lamports() < nft_vault.mint_price {
    return Err(ErrorCode::NotEnoughSol.into())
  }

  system_program::transfer(
    CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: ctx.accounts.buyer.to_account_info().clone(),
            to: ctx.accounts.nft_vault_pool.to_account_info(),
        },
    ),
    nft_vault.mint_price,
  )?;

  let vault_address = nft_vault.key().clone();
  let seeds = [
    NFT_VAULT_POOL_SEED.as_bytes(),
    vault_address.as_ref(),
    &[nft_vault.pool_bump]
  ];

  let cpi_context = CpiContext::new(
    ctx.accounts.token_program.to_account_info().clone(),
    Transfer {
      from: ctx.accounts.vault_pool_ata.to_account_info().clone(),
      to: ctx.accounts.buyer_ata.to_account_info().clone(),
      authority: ctx.accounts.nft_vault_pool.to_account_info().clone(),
    }
  );
  transfer(cpi_context.with_signer(&[&seeds[..]]), 1)?;

  nft_vault.sold_mints.push(ctx.accounts.nft_mint.key());
  
  msg!("Buyer: {:?}", ctx.accounts.buyer.key());
  msg!("NFT Mint: {:?}", ctx.accounts.nft_mint.key());
  msg!("Minted NFTS: {:?}", nft_vault.sold_mints.len());

  Ok(())
}