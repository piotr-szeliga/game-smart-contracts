mod state;
mod ins;

use anchor_lang::prelude::*;
use state::ins::*;


declare_id!("4NtgP5gmSJFPoifmXS1Fw1zHGvwhguRNhzT5jP7u3U6A");

#[program]
pub mod gift {
    use super::*;

    pub fn create_gift(ctx: Context<CreateGift>) -> Result<()> {
        Ok(())
    }
}

