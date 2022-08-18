use anchor_lang::prelude::*;
use market_accounts::{
    OrbitMarketAccount,
    program::OrbitMarketAccounts
};
use crate::{
    DigitalTransaction,
    DigitalProduct,
    BuyerDecisionState,
};
use transaction::transaction_struct::TransactionState;

#[derive(Accounts)]
pub struct OpenDigitalTransactionSol<'info>{
    #[account(
        init,
        space = 2000,
        payer = buyer_wallet,
    )]
    pub digital_transaction: Box<Account<'info, DigitalTransaction>>,

    #[account(
        constraint = digital_product.metadata.currency == System::id()
    )]
    pub digital_product: Account<'info, DigitalProduct>,

    #[account(
        seeds = [
            b"orbit_escrow_account",
            digital_transaction.key().as_ref()
        ],
        bump
    )]
    pub escrow_account: SystemAccount<'info>,

    pub buyer_account: Account<'info, OrbitMarketAccount>,

    #[account(mut)]
    pub buyer_wallet: Signer<'info>,

    #[account(
        address = buyer_account.master_pubkey
    )]
    pub buyer_auth: Signer<'info>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct CloseDigitalTransactionSol<'info>{
    #[account(
        mut,
        constraint = digital_transaction.metadata.transaction_state == TransactionState::BuyerConfirmedProduct,
        constraint = digital_transaction.final_decision != BuyerDecisionState::Null,
    )]
    pub digital_transaction: Box<Account<'info, DigitalTransaction>>,

    #[account(
        address = digital_transaction.metadata.buyer
    )]
    pub buyer_account: Account<'info, OrbitMarketAccount>,

    #[account(
        mut,
        address = buyer_account.wallet
    )]
    pub buyer_wallet: SystemAccount<'info>,

    #[account(
        address = digital_transaction.metadata.seller
    )]
    pub seller_account: Account<'info, OrbitMarketAccount>,

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
        constraint = (authority.key() == seller_account.master_pubkey) || (authority.key() == buyer_account.master_pubkey)
    )]
    pub authority: Signer<'info>,

    #[account(
        seeds = [b"digital_auth"],
        bump
    )]
    pub digital_auth: SystemAccount<'info>,

    #[account(
        address = market_accounts::ID
    )]
    pub market_account_program: Program<'info, OrbitMarketAccounts>
}

#[derive(Accounts)]
pub struct FundEscrowSol<'info>{
    #[account(
        mut,
        constraint = digital_transaction.metadata.transaction_state == TransactionState::SellerConfirmed,
    )]
    pub digital_transaction: Box<Account<'info, DigitalTransaction>>,

    #[account(
        address = digital_transaction.metadata.buyer
    )]
    pub buyer_account: Account<'info, OrbitMarketAccount>,

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

    #[account(mut)]
    pub buyer_wallet: Signer<'info>,

    #[account(
        address = buyer_account.master_pubkey
    )]
    pub buyer_auth: Signer<'info>
}
