use anchor_lang::prelude::*;
use product::product_struct::OrbitProduct;

pub mod accessors;
pub mod structs;
pub mod errors;

pub use accessors::*;
pub use structs::*;
pub use errors::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod orbit_digital_market {
    use super::*;
    use product::product_trait::OrbitProductTrait;
    use transaction::transaction_trait::OrbitTransactionTrait;
    use market_accounts::structs::OrbitMarketAccountTrait;

    //////////////////////////
    /// TRANSACTION

    pub fn open_digital_transaction(ctx: Context<OpenDigitalTransaction>, price: u64) -> Result<()>{
        DigitalTransaction::open(ctx, price)
    }
    pub fn close(ctx: Context<CloseDigitalTransaction>) -> Result<()>{
        DigitalTransaction::close(ctx)
    }
    pub fn fund_escrow(ctx: Context<FundEscrow>) -> Result<()>{
        DigitalTransaction::fund_escrow(ctx)
    }

    pub fn close_transaction_account(ctx: Context<CloseTransactionAccount>) -> Result<()>{
        DigitalTransaction::close_transaction_account(ctx)
    }
    
    pub fn confirm_delivered(ctx: Context<BuyerConfirmation>) -> Result<()>{
        confirm_delivered_handler(ctx)
    }
    pub fn confirm_accept(ctx: Context<BuyerConfirmation>) -> Result<()>{
        confirm_accept_handler(ctx)
    }
    pub fn deny_accept(ctx: Context<BuyerConfirmation>) -> Result<()>{
        deny_accept_handler(ctx)
    }

    pub fn commit_keys(ctx: Context<CommitKeys>, keys: Vec<u8>, link: [u8; 64]) -> Result<()>{
        commit_keys_handler(ctx, keys, link)
    }

    pub fn seller_accept_transaction(ctx: Context<SellerAcceptTransaction>) -> Result<()>{
        seller_accept_transaction_handler(ctx)
    }

    //////////////////////////////
    /// PRODUCT

    pub fn list_digital_product(ctx: Context<ListDigitalProduct>, prod: OrbitProduct)-> Result<()>{
        DigitalProduct::list(ctx, prod)
    }
    pub fn unlist_digital_product(ctx: Context<UnlistDigitalProduct>)-> Result<()>{
        DigitalProduct::unlist(ctx)
    }

    pub fn set_product_type(ctx: Context<SetDigitalProductField>, prod_type: DigitalProductType) -> Result<()>{
        set_product_type_handler(ctx, prod_type)
    }
    pub fn set_file_type(ctx: Context<SetDigitalProductField>, file_type: DigitalFileTypes) -> Result<()>{
        set_file_type_handler(ctx, file_type)
    }
    pub fn change_availability(ctx: Context<SetDigitalProductField>, available: bool) -> Result<()>{
        change_availability_handler(ctx, available)
    }
    pub fn change_price(ctx: Context<SetDigitalProductField>, price: u64) -> Result<()>{
        change_price_handler(ctx, price)
    }

    ////////////////////////////////
    /// COMMISSION RELATED
    
    pub fn create_commission_account(ctx: Context<CreateComishAccount>) -> Result<()>{
        create_commission_account_handler(ctx)
    }
    pub fn commit_preview(ctx: Context<CommitPreview>, link: [u8; 64]) -> Result<()>{
        commit_preview_handler(ctx, link)
    }
    pub fn propose_rate(ctx: Context<UpdateRate>, new_rate: u8) -> Result<()>{
        propose_rate_handler(ctx, new_rate)
    }
    pub fn accept_rate(ctx: Context<UpdateRate>) -> Result<()>{
        accept_rate_handler(ctx)
    }

    /////////////////////////////////////////////////
    /// REVIEW RELATED

    pub fn leave_review(ctx: Context<LeaveReview>, rating: u8) -> Result<()>{
        DigitalTransaction::leave_review(ctx, rating)
    }
}

