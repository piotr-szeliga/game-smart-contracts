use anchor_lang::{
  prelude::*, 
  solana_program::{
    clock,
    keccak::{hash, Hash}
  }
};
use crate::constants::*;

pub fn now() -> u64 {
  clock::Clock::get().unwrap().unix_timestamp as u64
}

pub fn get_random() -> u32 {
  let timestamp = now();
  let hash = hash(&timestamp.to_be_bytes());
  let buf: [u8; 32] = Hash::to_bytes(hash);
  let slice: [u8; 4] = [buf[10], buf[12], buf[8], buf[16]];
  u32::from_be_bytes(slice)  
}

pub fn get_status(bet_amount: u8, bet_number: u8, win_percents: [u16; 6]) -> (u32, u64) {
  let mut rand = get_random();
  let bn = bet_amount as usize;
  let price = BET_PRICES[bn];
  
  rand = rand % 10000;
  let mut earned: u64 = 0;
  if rand < win_percents[bn].into() && rand % 2 == bet_number.into() {
    earned = price.checked_mul(2).unwrap();
  }
  
  msg!("Random: {:?}", rand);
  msg!("Bet Price: {:?}", BET_PRICES[bn]);
  msg!("Bet Number: {:?}", bet_number);
  msg!("Earned: {:?}", earned);

  return (rand, earned);
}