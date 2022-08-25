use anchor_lang::{
    prelude::*,
    AccountsClose
};
use market_accounts::OrbitMarketAccount;
use product::{product_trait::OrbitProductTrait, product_struct::OrbitProduct};
use crate::{DigitalProduct, DigitalProductType, DigitalFileTypes, DigitalMarketErrors};

#[derive(Accounts)]
pub struct ListDigitalProduct<'info>{
    
    #[account(
        init,
        space = 200,
        payer = seller_wallet
    )]
    pub digital_product: Account<'info, DigitalProduct>,

    pub seller_account: Account<'info, OrbitMarketAccount>,

    #[account(
        mut,
        address = seller_account.wallet
    )]
    pub seller_wallet: Signer<'info>,

    pub system_program: Program<'info, System>
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

impl<'a, 'b> OrbitProductTrait<'a, 'b, ListDigitalProduct<'a>, UnlistDigitalProduct<'b>> for DigitalProduct{
    fn list(ctx: Context<ListDigitalProduct>, prod: OrbitProduct)-> Result<()> {
        if prod.seller != ctx.accounts.seller_account.key() {
            return err!(DigitalMarketErrors::InvalidSellerForListing)
        }
        ctx.accounts.digital_product.metadata = prod;
        Ok(())
    }

    fn unlist(ctx: Context<UnlistDigitalProduct>)-> Result<()> {
        ctx.accounts.digital_product.close(ctx.accounts.seller_wallet.to_account_info())
    }
}

#[derive(Accounts)]
pub struct SetDigitalProductField<'info>{

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

pub fn set_product_type_handler(ctx: Context<SetDigitalProductField>, prod_type: DigitalProductType) -> Result<()>{
    ctx.accounts.digital_product.digital_product_type = prod_type;   
    Ok(())
}

pub fn set_file_type_handler(ctx: Context<SetDigitalProductField>, file_type: DigitalFileTypes) -> Result<()>{
    ctx.accounts.digital_product.digital_file_type = file_type;   
    Ok(())
}

pub fn change_availability_handler(ctx: Context<SetDigitalProductField>, available: bool) -> Result<()>{
    ctx.accounts.digital_product.metadata.available = available;
    Ok(())
}

pub fn change_price_handler(ctx: Context<SetDigitalProductField>, price: u64) -> Result<()>{
    ctx.accounts.digital_product.metadata.price = price;
    Ok(())
}
pub fn update_currency_handler(ctx: Context<SetDigitalProductField>, currency: Pubkey) -> Result<()>{
    ctx.accounts.digital_product.metadata.currency = currency;
    Ok(())
}