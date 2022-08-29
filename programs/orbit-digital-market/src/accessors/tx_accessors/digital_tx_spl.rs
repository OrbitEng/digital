use anchor_lang::prelude::*;
use market_accounts::{
    OrbitMarketAccount,
    program::OrbitMarketAccounts
};
use crate::{
    DigitalTransaction,
    DigitalProduct,
    BuyerDecisionState, program::OrbitDigitalMarket,
};
use transaction::transaction_struct::TransactionState;
use anchor_spl::token::{
    TokenAccount,
    Mint,
    Token
};

#[derive(Accounts)]
pub struct OpenDigitalTransactionSpl<'info>{
    #[account(
        init,
        space = 2000,
        payer = buyer_wallet,
    )]
    pub digital_transaction: Box<Account<'info, DigitalTransaction>>,

    #[account(
        constraint = digital_product.metadata.currency != System::id()
    )]
    pub digital_product: Account<'info, DigitalProduct>,

    #[account(
        address = digital_product.metadata.currency
    )]
    pub token_mint: Account<'info, Mint>,

    #[account(
        init,
        token::mint = token_mint,
        token::authority = digital_auth,
        seeds = [
            b"digital_escrow_spl",
            digital_transaction.key().as_ref(),
        ],
        bump,
        payer = buyer_wallet
    )]
    pub escrow_account: Account<'info, TokenAccount>,

    pub buyer_account: Account<'info, OrbitMarketAccount>,

    #[account(mut)]
    pub buyer_wallet: Signer<'info>,

    #[account(
        address = buyer_account.master_pubkey
    )]
    pub buyer_auth: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    #[account(
        seeds = [b"market_authority"],
        bump
    )]
    pub digital_auth: SystemAccount<'info>,

    pub rent: Sysvar<'info, Rent>
}

#[derive(Accounts)]
pub struct CloseDigitalTransactionSpl<'info>{
    #[account(
        mut,
        constraint =    (digital_transaction.metadata.transaction_state == TransactionState::BuyerConfirmedProduct) ||
                        (digital_transaction.metadata.transaction_state == TransactionState::Opened),
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
        constraint = seller_token_account.owner == seller_account.wallet
    )]
    pub seller_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [
            b"digital_escrow_spl",
            digital_transaction.key().as_ref(),
        ],
        bump,

        address = digital_transaction.metadata.escrow_account
    )]
    pub escrow_account: Account<'info, TokenAccount>,

    #[account(
        constraint = (authority.key() == seller_account.master_pubkey) || (authority.key() == buyer_account.master_pubkey)
    )]
    pub authority: Signer<'info>,

    #[account(
        seeds = [b"market_authority"],
        bump
    )]
    pub digital_auth: SystemAccount<'info>,

    #[account(
        address = market_accounts::ID
    )]
    pub market_account_program: Program<'info, OrbitMarketAccounts>,

    pub token_program: Program<'info, Token>,

    pub digital_program: Program<'info, OrbitDigitalMarket>,
}

#[derive(Accounts)]
pub struct FundEscrowSpl<'info>{
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
            b"digital_escrow_spl",
            digital_transaction.key().as_ref(),
        ],
        bump,

        address = digital_transaction.metadata.escrow_account
    )]
    pub escrow_account: Account<'info, TokenAccount>,

    #[account(
        address = buyer_account.master_pubkey
    )]
    pub buyer_auth: Signer<'info>,

    #[account(
        address = buyer_spl_wallet.owner
    )]
    pub wallet_owner: Signer<'info>,

    pub buyer_spl_wallet: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>
}
