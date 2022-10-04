use anchor_lang::{
    prelude::*,
    AccountsClose
};
use orbit_catalog::{
    structs::{
        OrbitModCatalogStruct,
        OrbitVendorCatalog
    },
    cpi::{
        accounts::{
            EditModCatalog,
            ModifyVendorCatalog
        },
        edit_mod_catalog, list_product, unlist_product
    }, program::OrbitCatalog
};
use product::{product_struct::OrbitProduct, product_trait::OrbitProductTrait, CommonProdUtils};
use crate::{DigitalProduct, DigitalFileTypes, DigitalMarketErrors, program::OrbitDigitalMarket};

#[derive(Accounts)]
#[instruction(prod_in: OrbitProduct)]
pub struct ListDigitalProduct<'info>{
    
    #[account(
        init,
        space = 200,
        payer = seller_wallet,
        seeds = [
            b"commission_product",
            vendor_catalog.key().as_ref(),
            &[prod_in.index]
        ],
        bump
    )]
    pub digital_product: Box<Account<'info, DigitalProduct>>,

    #[account(
        address = prod_in.owner_catalog
    )]
    pub vendor_catalog: Account<'info, OrbitVendorCatalog>,

    #[account(
        mut,
        address = vendor_catalog.catalog_owner
    )]
    pub seller_wallet: Signer<'info>,

    pub system_program: Program<'info, System>,

    #[account(
        mut,
        seeds = [
            b"recent_catalog"
        ],
        bump
    )]
    pub recent_catalog: Box<Account<'info, OrbitModCatalogStruct>>,

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
        address = digital_product.metadata.owner_catalog
    )]
    pub vendor_catalog: Account<'info, OrbitVendorCatalog>,

    #[account(
        mut,
        address = vendor_catalog.catalog_owner
    )]
    pub seller_wallet: Signer<'info>,

    pub catalog_program: Program<'info, OrbitCatalog>,
}

impl <'a, 'b> OrbitProductTrait<'a, 'b, ListDigitalProduct<'a>, UnlistDigitalProduct<'b>> for DigitalProduct{
    fn list(ctx: Context<ListDigitalProduct>, prod: OrbitProduct)-> Result<()> {
        if prod.owner_catalog != ctx.accounts.vendor_catalog.key() {
            return err!(DigitalMarketErrors::InvalidSellerForListing)
        }
        
        list_product(
            CpiContext::new(
                ctx.accounts.catalog_program.to_account_info(),
                ModifyVendorCatalog{
                    vendor_catalog: ctx.accounts.vendor_catalog.to_account_info(),
                    wallet: ctx.accounts.seller_wallet.to_account_info()
                }
            ), 
            prod.index.clone()
        ).expect("could not list product, index is taken");

        ctx.accounts.digital_product.metadata = prod;
        match ctx.bumps.get("market_auth"){
            Some(auth_bump) => edit_mod_catalog(
                CpiContext::new_with_signer(
                    ctx.accounts.catalog_program.to_account_info(),
                    EditModCatalog {
                        catalog: ctx.accounts.recent_catalog.to_account_info(),
                        product: ctx.accounts.digital_product.to_account_info(),
                        caller_auth: ctx.accounts.market_auth.to_account_info()
                    },
                    &[&[b"market_auth", &[*auth_bump]]])
            ),
            None => err!(DigitalMarketErrors::InvalidAuthBump)
        }
    }

    fn unlist(ctx: Context<UnlistDigitalProduct>)-> Result<()> {
        unlist_product(
            CpiContext::new(
                ctx.accounts.catalog_program.to_account_info(),
                ModifyVendorCatalog{
                    vendor_catalog: ctx.accounts.vendor_catalog.to_account_info(),
                    wallet: ctx.accounts.seller_wallet.to_account_info()
                }
            ),
            ctx.accounts.digital_product.metadata.index
        ).expect("could not update catalog");
        ctx.accounts.digital_product.close(ctx.accounts.seller_wallet.to_account_info())
    }
}

#[derive(Accounts, CommonProdUtils)]
pub struct UpdateProductField<'info>{
    #[account(mut)]
    pub digital_product: Account<'info, DigitalProduct>,

    #[account(
        address = digital_product.metadata.owner_catalog
    )]
    pub vendor_catalog: Account<'info, OrbitVendorCatalog>,

    #[account(
        mut,
        address = vendor_catalog.catalog_owner
    )]
    pub seller_wallet: Signer<'info>,
}

pub fn set_file_type_handler(ctx: Context<UpdateProductField>, file_type: DigitalFileTypes) -> Result<()>{
    ctx.accounts.digital_product.digital_file_type = file_type;   
    Ok(())
}