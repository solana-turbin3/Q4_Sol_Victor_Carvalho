use anchor_lang::prelude::*;

declare_id!("3vJaEJMGZ6yiRtpxgctoxqnsUu73PRUJP5tJAUyCVPRX");

pub mod state;
pub mod instructions;
pub mod errors;

pub use instructions::*;
pub use errors::*;

#[program]
pub mod solana {
    use super::*;

    /////////////////////////////
    /// FarmLink Instructions ///
    /////////////////////////////

    // Initialize FarmLink
    pub fn initialize(ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.initialize(name, fee, &ctx.bumps)
    }

    // Create product
    pub fn create_product(ctx: Context<CreateProduct>, price: u64) -> Result<()> {
        ctx.accounts.create_product(price, &ctx.bumps)?;
        ctx.accounts.deposit_token_to_vault()?;
        Ok(())
    }

    // Purchase product
    pub fn purchase_product(ctx: Context<Purchase>) -> Result<()> {
        ctx.accounts.send_sol_to_vault()?;
        ctx.accounts.send_token_to_consumer()?;
        ctx.accounts.close_mint_vault()?;
        Ok(())
    }

    // Deliver product
    pub fn deliver_product(ctx: Context<Deliver>) -> Result<()> {
        ctx.accounts.burn_consumer_token()?;
        ctx.accounts.transfer_vault_funds_to_farmer()?;
        Ok(())
    }

    // Refund consumer

    // Refund farmer

}