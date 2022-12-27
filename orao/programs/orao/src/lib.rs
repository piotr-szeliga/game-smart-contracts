mod ins;
mod state;
mod constants;

use anchor_lang::prelude::*;
use crate::ins::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod orao {
    use super::*;

    pub fn spin_and_pull_the_trigger(ctx: Context<SpinAndPullTheTrigger>) -> Result<()> {
        // Zero seed is illegal in VRF
        if force == [0_u8; 32] {
            return Err(Error::YouMustSpinTheCylinder.into());
        }

        let player_state = &mut ctx.accounts.player_state;

        // initialize
        if player_state.rounds == 0 {
            **player_state = PlayerState::new(*ctx.accounts.player.as_ref().key);
        }

        // Assert that the player is able to play.
        player_state.assert_can_play(ctx.accounts.prev_round.as_ref())?;

        let cpi_program = ctx.accounts.vrf.to_account_info();
        let cpi_accounts = orao_solana_vrf::cpi::accounts::Request {
            payer: ctx.accounts.player.to_account_info(),
            network_state: ctx.accounts.config.to_account_info(),
            treasury: ctx.accounts.treasury.to_account_info(),
            request: ctx.accounts.random.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        };
        let cpi_ctx = anchor_lang::context::CpiContext::new(cpi_program, cpi_accounts);
        orao_solana_vrf::cpi::request(cpi_ctx, force)?;

        player_state.rounds += 1;
        player_state.force = force;

        Ok(())
    }
}
