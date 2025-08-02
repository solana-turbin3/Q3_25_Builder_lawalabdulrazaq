use anchor_lang::prelude::*;

declare_id!("5CmMY73bbqth3sqK8r9MmFVhZW7qSbkkG9LTuHzipbCW");

#[program]
pub mod dexzikon {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
