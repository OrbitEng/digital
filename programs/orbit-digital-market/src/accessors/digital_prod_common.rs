use anchor_lang::{
    prelude::*,
    AccountsClose
};
use orbit_catalog::{
    structs::OrbitModCatalogStruct,
    cpi::{
        accounts::EditModCatalog,
        edit_mod_catalog
    }, program::OrbitCatalog
};
use market_accounts::OrbitMarketAccount;
use product::product_struct::OrbitProduct;
use crate::{DigitalProduct, DigitalFileTypes, DigitalMarketErrors, program::OrbitDigitalMarket, DigitalProductType};

#[derive(Accounts)]
pub struct ListDigitalProductCommission<'info>{
    
    #[account(
        init,
        space = 200,
        payer = seller_wallet
    )]
    pub digital_product: Box<Account<'info, DigitalProduct>>,

    pub seller_account: Box<Account<'info, OrbitMarketAccount>>,

    #[account(
        mut,
        address = seller_account.wallet
    )]
    pub seller_wallet: Signer<'info>,

    pub system_program: Program<'info, System>,

    #[account(
        mut,
        seeds = [
            b"recent_commission_catalog"
        ],
        bump
    )]
    pub recent_commission_catalog: Box<Account<'info, OrbitModCatalogStruct>>,

    #[account(
        seeds = [
            b"market_auth"
        ],
        bump
    )]
    pub market_auth: SystemAccount<'info>,

    pub catalog_program: Program<'info, OrbitCatalog>,

    pub digital_program: Program<'info, OrbitDigitalMarket>,
}

#[derive(Accounts)]
pub struct ListDigitalProductTemplate<'info>{
    
    #[account(
        init,
        space = 200,
        payer = seller_wallet
    )]
    pub digital_product: Box<Account<'info, DigitalProduct>>,

    pub seller_account: Box<Account<'info, OrbitMarketAccount>>,

    #[account(
        mut,
        address = seller_account.wallet
    )]
    pub seller_wallet: Signer<'info>,

    pub system_program: Program<'info, System>,

    #[account(
        mut,
        seeds = [
            b"recent_template_catalog"
        ],
        bump
    )]
    pub recent_template_catalog: Box<Account<'info, OrbitModCatalogStruct>>,

    #[account(
        seeds = [
            b"market_auth"
        ],
        bump
    )]
    pub market_auth: SystemAccount<'info>,

    pub catalog_program: Program<'info, OrbitCatalog>,

    pub digital_program: Program<'info, OrbitDigitalMarket>,
}

#[derive(Accounts)]
pub struct UnlistDigitalProduct<'info>{

    #[account(mut)]
    pub digital_product: Account<'info, DigitalProduct>,

    #[account(
        address = digital_product.metadata.seller
    )]
    pub seller_account: Account<'info, OrbitMarketAccount>,

    #[account(
        mut,
        address = seller_account.wallet
    )]
    pub seller_wallet: Signer<'info>,
}

pub fn list_template_handler(ctx: Context<ListDigitalProductTemplate>, prod: OrbitProduct)-> Result<()> {
    if prod.seller != ctx.accounts.seller_account.key() {
        return err!(DigitalMarketErrors::InvalidSellerForListing)
    }
    ctx.accounts.digital_product.metadata = prod;
    ctx.accounts.digital_product.digital_product_type = DigitalProductType::Template;
    match ctx.bumps.get("market_auth"){
        Some(auth_bump) => edit_mod_catalog(
            CpiContext::new_with_signer(
                ctx.accounts.catalog_program.to_account_info(),
                EditModCatalog {
                    catalog: ctx.accounts.recent_template_catalog.to_account_info(),
                    product: ctx.accounts.digital_product.to_account_info(),
                    caller_auth: ctx.accounts.market_auth.to_account_info()
                },
                &[&[b"market_auth", &[*auth_bump]]])
        ),
        None => err!(DigitalMarketErrors::InvalidAuthBump)
    }
}

pub fn list_commission_handler(ctx: Context<ListDigitalProductCommission>, prod: OrbitProduct)-> Result<()> {
    if prod.seller != ctx.accounts.seller_account.key() {
        return err!(DigitalMarketErrors::InvalidSellerForListing)
    }
    ctx.accounts.digital_product.metadata = prod;
    ctx.accounts.digital_product.digital_product_type = DigitalProductType::Commission;
    match ctx.bumps.get("market_auth"){
        Some(auth_bump) => edit_mod_catalog(
            CpiContext::new_with_signer(
                ctx.accounts.catalog_program.to_account_info(),
                EditModCatalog {
                    catalog: ctx.accounts.recent_commission_catalog.to_account_info(),
                    product: ctx.accounts.digital_product.to_account_info(),
                    caller_auth: ctx.accounts.market_auth.to_account_info()
                },
                &[&[b"market_auth", &[*auth_bump]]])
        ),
        None => err!(DigitalMarketErrors::InvalidAuthBump)
    }
}

pub fn unlist_product_handler(ctx: Context<UnlistDigitalProduct>)-> Result<()> {
    ctx.accounts.digital_product.close(ctx.accounts.seller_wallet.to_account_info())
}

#[derive(Accounts)]
pub struct UpdateProductField<'info>{

    #[account(mut)]
    pub digital_product: Account<'info, DigitalProduct>,

    #[account(
        address = digital_product.metadata.seller
    )]
    pub seller_account: Account<'info, OrbitMarketAccount>,

    #[account(
        mut,
        address = seller_account.master_pubkey
    )]
    pub seller_auth: Signer<'info>
}

pub fn set_file_type_handler(ctx: Context<UpdateProductField>, file_type: DigitalFileTypes) -> Result<()>{
    ctx.accounts.digital_product.digital_file_type = file_type;   
    Ok(())
}

pub fn change_availability_handler(ctx: Context<UpdateProductField>, available: bool) -> Result<()>{
    ctx.accounts.digital_product.metadata.available = available;
    Ok(())
}

/// GENERAL

pub fn update_price_handler(ctx: Context<UpdateProductField>, price: u64) -> Result<()>{
    ctx.accounts.digital_product.metadata.price = price;
    Ok(())
}

pub fn update_currency_handler(ctx: Context<UpdateProductField>, currency: Pubkey) -> Result<()>{
    ctx.accounts.digital_product.metadata.currency = currency;
    Ok(())
}

pub fn update_media_handler(ctx: Context<UpdateProductField>, link: String) -> Result<()>{
    ctx.accounts.digital_product.metadata.media = link;
    Ok(())
}

pub fn update_info_handler(ctx: Context<UpdateProductField>, info: String) -> Result<()>{
    ctx.accounts.digital_product.metadata.info = info;
    Ok(())
}