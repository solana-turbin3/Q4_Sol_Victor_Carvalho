use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Refund<'info> {

}

impl<'info> Refund<'info> {
    pub fn refund_consumer(&self) -> Result<()> {
        Ok(())
    }

    pub fn refund_farmer(&self) -> Result<()> {
        Ok(())
    }
}