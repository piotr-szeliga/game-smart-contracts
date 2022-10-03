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

pub fn get_status(price: u64) -> (u32, u64) {
  let mut rand = get_random();
  
  let mut max = rand % 2 + 1;
  rand = rand % 100;
  if rand < 25 {
      max = 3;
  }
  if rand < 20 {
      max = 4;
  }
  if rand < 10 {
      max = 5;
  }

  let earned = match max {
      3 => price,
      4 => price.checked_mul(5).unwrap().checked_div(4).unwrap(),
      5 => price.checked_mul(3).unwrap().checked_div(2).unwrap(),
      _ => 0,
  };

  msg!("Status: {:?}", rand);
  msg!("Max Equal: {:?}", max);

  return (rand, earned);
}