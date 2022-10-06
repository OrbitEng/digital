use anchor_lang::prelude::*;
use crate::{DigitalMarketErrors, program::OrbitDigitalMarket};
use orbit_catalog::{cpi::{
    accounts::CreateMarketCatalog,
    init_market_catalog
}, program::OrbitCatalog};

#[derive(Accounts)]
pub struct CreateDigitalRecentCatalog<'info>{
    #[account(
        mut,
        seeds = [
            b"recent_catalog"
        ],
        bump
    )]
    pub catalog: SystemAccount<'info>,
    
    pub self_program: Program<'info, OrbitDigitalMarket>,

    #[account(
        seeds = [
            b"market_auth"
        ],
        bump
    )]
    pub market_auth: SystemAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub catalog_program: Program<'info, OrbitCatalog>,

    pub system_program: Program<'info, System>
}

pub fn recent_digital_catalog_handler(ctx: Context<CreateDigitalRecentCatalog>) -> Result<()>{
    
    if let Some(catalog_bump) = ctx.bumps.get("catalog"){
        init_market_catalog(
            CpiContext::new_with_signer(
                ctx.accounts.catalog_program.to_account_info(),
                CreateMarketCatalog {
                    catalog: ctx.accounts.catalog.to_account_info(),
                    market_auth: ctx.accounts.market_auth.to_account_info(),
                    invoker: ctx.accounts.self_program.to_account_info(),
                    payer: ctx.accounts.payer.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info()
                },
                &[&[b"recent_catalog", &[*catalog_bump]]]
            )
        )
    }else{
        return err!(DigitalMarketErrors::InvalidAuthBump)
    }

}