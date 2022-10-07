use anchor_lang::prelude::*;
use market_accounts::{
    OrbitMarketAccount,
    program::OrbitMarketAccounts
};
use orbit_catalog::OrbitVendorCatalog;
use orbit_multisig::Multisig;
use crate::{
    DigitalTransaction,
    DigitalProduct,
    BuyerDecisionState, program::OrbitDigitalMarket,
};
use transaction::transaction_struct::TransactionState;

#[derive(Accounts)]
pub struct OpenDigitalTransactionSol<'info>{
    #[account(
        init,
        space = 3000,
        payer = buyer_wallet,
    )]
    pub digital_transaction: Box<Account<'info, DigitalTransaction>>,

    #[account(
        constraint = digital_product.metadata.currency == System::id()
    )]
    pub digital_product: Box<Account<'info, DigitalProduct>>,

    #[account(
        constraint = seller_account.wallet == seller_catalog.catalog_owner
    )]
    pub seller_account:Box<Account<'info, OrbitMarketAccount>>,

    #[account(
        address = digital_product.metadata.owner_catalog
    )]
    pub seller_catalog:Box<Account<'info, OrbitVendorCatalog>>,

    #[account(
        seeds = [
            b"orbit_escrow_account",
            digital_transaction.key().as_ref()
        ],
        bump
    )]
    pub escrow_account: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [
            b"orbit_account",
            buyer_wallet.key().as_ref()
        ],
        bump,
        seeds::program = market_accounts::ID
    )]
    pub buyer_account:Box<Account<'info, OrbitMarketAccount>>,

    #[account(
        mut,
        address = buyer_account.wallet
    )]
    pub buyer_wallet: Signer<'info>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct CloseDigitalTransactionSol<'info>{
    #[account(
        mut,
        constraint =    ((digital_transaction.metadata.transaction_state == TransactionState::BuyerConfirmedProduct) && (digital_transaction.final_decision != BuyerDecisionState::Null)) ||
                        (digital_transaction.metadata.transaction_state == TransactionState::Opened),
    )]
    pub digital_transaction: Box<Account<'info, DigitalTransaction>>,

    #[account(
        constraint = buyer_account.key() == digital_transaction.metadata.buyer
    )]
    pub buyer_account:Box<Account<'info, OrbitMarketAccount>>,

    #[account(
        mut,
        address = buyer_account.wallet
    )]
    pub buyer_wallet: SystemAccount<'info>,

    #[account(
        constraint = seller_account.key() == digital_transaction.metadata.seller
    )]
    pub seller_account:Box<Account<'info, OrbitMarketAccount>>,

    #[account(
        mut,
        address = seller_account.wallet
    )]
    pub seller_wallet: SystemAccount<'info>,

    #[account(
        seeds = [
            b"orbit_escrow_account",
            digital_transaction.key().as_ref()
        ],
        bump,

        address = digital_transaction.metadata.escrow_account
    )]
    pub escrow_account: SystemAccount<'info>,

    #[account(
        seeds = [b"market_authority"],
        bump
    )]
    pub digital_auth: SystemAccount<'info>,

    #[account(
        address = market_accounts::ID
    )]
    pub market_account_program: Program<'info, OrbitMarketAccounts>,

    pub digital_program: Program<'info, OrbitDigitalMarket>,

    #[account(
        mut,
        address = Pubkey::new(orbit_addresses::MULTISIG_WALLET_ADDRESS)
    )]
    pub multisig_address: Box<Account<'info, Multisig>>,
    
    #[account(
        mut,
        seeds = [
            multisig_address.key().as_ref()
        ],
        bump = multisig_address.nonce
    )]
    pub multisig_wallet: SystemAccount<'info>,
}

#[derive(Accounts)]
pub struct FundEscrowSol<'info>{
    #[account(
        mut,
        constraint = digital_transaction.metadata.transaction_state == TransactionState::SellerConfirmed,
    )]
    pub digital_transaction: Box<Account<'info, DigitalTransaction>>,

    #[account(
        constraint = buyer_account.key() == digital_transaction.metadata.buyer,
        seeds = [
            b"orbit_account",
            buyer_wallet.key().as_ref()
        ],
        bump,
        seeds::program = market_accounts::ID
    )]
    pub buyer_account:Box<Account<'info, OrbitMarketAccount>>,

    #[account(
        mut,
        seeds = [
            b"orbit_escrow_account",
            digital_transaction.key().as_ref()
        ],
        bump,

        address = digital_transaction.metadata.escrow_account
    )]
    pub escrow_account: SystemAccount<'info>,

    #[account(
        mut,
        address = buyer_account.wallet
    )]
    pub buyer_wallet: Signer<'info>,
}
