use anchor_lang::prelude::*;

declare_id!("EFuXqksAXmMK571ax8idTu7XE4REAJSJkaN3rU3u4p3D");

#[program]
pub mod loyonsol {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
