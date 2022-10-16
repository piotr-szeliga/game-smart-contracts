use anchor_lang::{
  prelude::*, 
  solana_program::{
    clock,
    keccak::{hash, Hash}
  }
};

pub fn get_random() -> u32 {
  let slot = clock::Clock::get().unwrap().slot;
  let hash = hash(&slot.to_be_bytes());
  let buf: [u8; 32] = Hash::to_bytes(hash);
  let slice: [u8; 4] = [buf[10], buf[12], buf[8], buf[16]];
  u32::from_be_bytes(slice)  
}

pub fn get_status(price: u64, win_percents: [u16; 3], jackpot: u64) -> (u32, u64) {
  let mut rand = get_random();
  
  let mut max = rand % 2 + 1;
  rand = rand % 10000;
  for i in 0..3 {
    if rand < win_percents[i].into() {
      max = 3 + i as u32;
    }
  }

  let multipler = (max - 1) * 10 - rand % 10;

  let earned = match max {
      3 => price.checked_mul(multipler as u64).unwrap().checked_div(10).unwrap(),
      4 => price.checked_mul(multipler as u64).unwrap().checked_div(10).unwrap(),
      5 => jackpot,
      _ => 0,
  };

  msg!("Status: {:?}", rand);
  msg!("Max Equal: {:?}", max);

  return (rand, earned);
}