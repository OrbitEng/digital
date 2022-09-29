use anchor_lang::prelude::*;
use market_accounts::{
    OrbitMarketAccount,
    program::OrbitMarketAccounts
};
use orbit_multisig::Multisig;
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
        space = 4000,
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

    #[account(
        mut,
        seeds = [
            b"orbit_account",
            buyer_wallet.key().as_ref()
        ],
        bump,
        seeds::program = market_accounts::ID
    )]
    pub buyer_account: Account<'info, OrbitMarketAccount>,

    #[account(
        mut,
        address = buyer_account.wallet
    )]
    pub buyer_wallet: Signer<'info>,

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
        constraint =    ((digital_transaction.metadata.transaction_state == TransactionState::BuyerConfirmedProduct) && (digital_transaction.final_decision != BuyerDecisionState::Null)) ||
                        (digital_transaction.metadata.transaction_state == TransactionState::Opened),
    )]
    pub digital_transaction: Box<Account<'info, DigitalTransaction>>,

    #[account(
        address = digital_transaction.metadata.buyer
    )]
    pub buyer_account: Box<Account<'info, OrbitMarketAccount>>,

    #[account(
        mut,
        address = buyer_account.wallet
    )]
    pub buyer_wallet: SystemAccount<'info>,

    #[account(
        mut,
        owner = buyer_account.wallet
    )]
    pub buyer_token_account: Account<'info, TokenAccount>,

    #[account(
        address = digital_transaction.metadata.seller
    )]
    pub seller_account: Box<Account<'info, OrbitMarketAccount>>,

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
        bump = multisig_address.nonce,
        seeds::program = orbit_multisig::ID
    )]
    pub multisig_owner: SystemAccount<'info>,

    #[account(
        constraint = multisig_ata.owner == multisig_owner.key()
    )]
    pub multisig_ata: Account<'info, TokenAccount>,
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
        address = buyer_account.wallet
    )]
    pub buyer_wallet: Signer<'info>,

    #[account(
        constraint = buyer_spl_wallet.owner == buyer_wallet.key()
    )]
    pub buyer_spl_wallet: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>
}
