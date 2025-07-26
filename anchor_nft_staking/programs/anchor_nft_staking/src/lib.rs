#![allow(unexpected_cfgs,deprecated)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("9Yr5kEzDiWeoWuxdKGx2q8oGUYc3SW458HtN6fe3eJQD");

#[program]
pub mod anchor_nft_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
