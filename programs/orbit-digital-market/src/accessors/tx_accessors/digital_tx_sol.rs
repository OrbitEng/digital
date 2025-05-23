use anchor_lang::prelude::*;
use market_accounts::{
    OrbitMarketAccount,
    program::OrbitMarketAccounts
};
use orbit_product::program::OrbitProduct;
use crate::{
    DigitalTransaction,
    BuyerDecisionState, program::OrbitDigitalMarket,
};
use orbit_transaction::{transaction_struct::TransactionState, BuyerOpenTransactions, program::OrbitTransaction, SellerOpenTransactions};
use orbit_product::DigitalProduct;

/////////////////////////////////
/// BASE TX UTILS

#[derive(Accounts)]
#[instruction(seller_tx_index: u8)]
pub struct OpenDigitalTransactionSol<'info>{
    ////////////////////////////////
    /// TX
    #[account(
        init,
        payer = buyer_wallet,
        space = 3000,
        seeds = [
            b"orbit_digital_transaction",
            seller_transactions_log.key().as_ref(),
            [seller_tx_index].as_ref()
        ],
        bump
    )]
    pub digital_transaction: Box<Account<'info, DigitalTransaction>>,
    
    #[account(
        seeds = [
            b"orbit_escrow_account",
            digital_transaction.key().as_ref(),
            buyer_transactions_log.key().as_ref()
        ],
        bump
    )]
    pub escrow_account: SystemAccount<'info>,

    #[account(
        mut,
        constraint = digital_product.metadata.owner_catalog == seller_market_account.voter_id
    )]
    pub digital_product: Box<Account<'info, DigitalProduct>>,
    
    //////////////////////////////////////////////////
    /// BUYER SELLER
    
    /// BUYER
    #[account(
        mut,
        seeds = [
            b"buyer_transactions",
            (&(orbit_transaction::TransactionType::Commissions).try_to_vec()?).as_slice(),
            &buyer_market_account.voter_id.to_le_bytes()
        ], 
        bump,
        seeds::program = &orbit_transaction::id()
    )]
    pub buyer_transactions_log: Box<Account<'info, BuyerOpenTransactions>>,

    #[account(
        mut
    )]
    pub buyer_market_account: Box<Account<'info, OrbitMarketAccount>>,

    #[account(
        mut,
        address = buyer_market_account.wallet
    )]
    pub buyer_wallet: Signer<'info>,
    
    /// SELLER
    
    #[account(    )]
    pub seller_market_account: Account<'info, OrbitMarketAccount>,

    #[account(
        mut,
        seeds = [
            b"seller_transactions",
            (&(orbit_transaction::TransactionType::Digital).try_to_vec()?).as_slice(),
            &seller_market_account.voter_id.to_le_bytes()
        ], 
        bump,
        seeds::program = &orbit_transaction::id()
    )]
    pub seller_transactions_log: Box<Account<'info, SellerOpenTransactions>>,

    /////////////////////////////////
    /// EXTRANEOUS
    
    #[account(
        seeds = [b"market_authority"],
        bump
    )]
    pub digital_auth: SystemAccount<'info>,
    
    pub digital_program: Program<'info, OrbitDigitalMarket>,

    pub transaction_program: Program<'info, OrbitTransaction>,

    pub market_account_program: Program<'info, OrbitMarketAccounts>,
    
    pub product_program: Program<'info, OrbitProduct>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CloseDigitalTransactionSol<'info>{
    ////////////////////////////////////////////
    /// TX
    #[account(
        mut,
        constraint =    ((digital_transaction.metadata.transaction_state == TransactionState::BuyerConfirmedProduct) && (digital_transaction.final_decision != BuyerDecisionState::Null)) ||
                        (digital_transaction.metadata.transaction_state == TransactionState::Opened),
    )]
    pub digital_transaction: Box<Account<'info, DigitalTransaction>>,
    
    #[account(
        mut,
        constraint = digital_product.metadata.index == digital_transaction.metadata.product,
        constraint = digital_product.metadata.owner_catalog == seller_account.voter_id
    )] 
    pub digital_product: Box<Account<'info, DigitalProduct>>,

    #[account(
        mut,
        seeds = [
            b"orbit_escrow_account",
            digital_transaction.key().as_ref(),
            buyer_transactions_log.key().as_ref()
        ],
        bump
    )]
    pub escrow_account: SystemAccount<'info>,

    ///////////////////////////////////////////////////
    /// BUYER SELLER ACCOUNTS
    
    /// BUYER
    #[account(
        mut,
        constraint = buyer_account.voter_id == digital_transaction.metadata.buyer
    )]
    pub buyer_account: Box<Account<'info, OrbitMarketAccount>>,
    
    #[account(
        mut,
        seeds = [
            b"buyer_transactions",
            (&(orbit_transaction::TransactionType::Digital).try_to_vec()?).as_slice(),
            &buyer_account.voter_id.to_le_bytes()
        ], 
        bump,
        seeds::program = &orbit_transaction::id()
    )]
    pub buyer_transactions_log: Box<Account<'info, BuyerOpenTransactions>>,

    #[account(
        mut,
        address = buyer_account.wallet
    )]
    pub buyer_wallet: SystemAccount<'info>,
    
    /// SELLER
    #[account(
        mut,
        constraint = seller_account.voter_id == digital_transaction.metadata.seller
    )]
    pub seller_account: Box<Account<'info, OrbitMarketAccount>>,

    #[account(
        mut,
        seeds = [
            b"seller_transactions",
            (&(orbit_transaction::TransactionType::Digital).try_to_vec()?).as_slice(),
            &seller_account.voter_id.to_le_bytes()
        ], 
        bump,
        seeds::program = &orbit_transaction::id()
    )]
    pub seller_transactions_log: Box<Account<'info, SellerOpenTransactions>>,

    #[account(
        mut,
        address = seller_account.wallet
    )]
    pub seller_wallet: SystemAccount<'info>,

    //////////////////////////////////
    /// CPI AND EXTRANEOUS
    
    #[account(
        mut,
        address = Pubkey::from(orbit_addresses::MULTISIG_SIGNER)
    )]
    pub multisig_wallet: SystemAccount<'info>,

    #[account(
        seeds = [b"market_authority"],
        bump
    )]
    pub digital_auth: SystemAccount<'info>,

    pub digital_program: Program<'info, OrbitDigitalMarket>,
    
    pub market_account_program: Program<'info, OrbitMarketAccounts>,

    pub transaction_program: Program<'info, OrbitTransaction>,
    
    pub product_program: Program<'info, OrbitProduct>,
}

#[derive(Accounts)]
pub struct FundEscrowSol<'info>{
    ////////////////////////////////////////////
    /// TX
    #[account(
        mut,
        constraint = digital_transaction.metadata.transaction_state == TransactionState::SellerConfirmed
    )]
    pub digital_transaction: Box<Account<'info, DigitalTransaction>>,
    
    #[account(
        mut,
        seeds = [
            b"orbit_escrow_account",
            digital_transaction.key().as_ref(),
            buyer_transactions_log.key().as_ref()
        ],
        bump
    )]
    pub escrow_account: SystemAccount<'info>,

    ////////////////////////////////////////////
    /// BUYER SELLER

    /// BUYER
    #[account(
        mut,
        seeds = [
            b"buyer_transactions",
            (&(orbit_transaction::TransactionType::Digital).try_to_vec()?).as_slice(),
            &buyer_market_account.voter_id.to_le_bytes()
        ], 
        bump,
        seeds::program = &orbit_transaction::id()
    )]
    pub buyer_transactions_log: Box<Account<'info, BuyerOpenTransactions>>,

    #[account(
        mut,
        constraint = buyer_market_account.voter_id == digital_transaction.metadata.buyer
    )]
    pub buyer_market_account: Box<Account<'info, OrbitMarketAccount>>,

    #[account(
        mut,
        address = buyer_market_account.wallet
    )]
    pub buyer_wallet: Signer<'info>
}

#[derive(Accounts)]
pub struct SellerEarlyDeclineSol<'info>{
    ////////////////////////////////////////////
    /// TX
    #[account(
        mut,
        constraint = digital_transaction.final_decision == BuyerDecisionState::Null
    )]
    pub digital_transaction: Box<Account<'info, DigitalTransaction>>,

    #[account(
        mut,
        seeds = [
            b"orbit_escrow_account",
            digital_transaction.key().as_ref(),
            buyer_transactions_log.key().as_ref()
        ],
        bump
    )]
    pub escrow_account: SystemAccount<'info>,

    ///////////////////////////////////////////////////
    /// BUYER SELLER ACCOUNTS
    
    /// BUYER
    #[account(
        mut,
        constraint = buyer_account.voter_id == digital_transaction.metadata.buyer
    )]
    pub buyer_account: Box<Account<'info, OrbitMarketAccount>>,
    
    #[account(
        mut,
        seeds = [
            b"buyer_transactions",
            (&(orbit_transaction::TransactionType::Digital).try_to_vec()?).as_slice(),
            &buyer_account.voter_id.to_le_bytes()
        ], 
        bump,
        seeds::program = &orbit_transaction::id()
    )]
    pub buyer_transactions_log: Box<Account<'info, BuyerOpenTransactions>>,

    #[account(
        mut,
        address = buyer_account.wallet
    )]
    pub buyer_wallet: SystemAccount<'info>,
    
    /// SELLER

    #[account(
        mut,
        constraint = seller_account.voter_id == digital_transaction.metadata.seller
    )]
    pub seller_account: Box<Account<'info, OrbitMarketAccount>>,
    
    #[account(
        mut,
        seeds = [
            b"seller_transactions",
            (&(orbit_transaction::TransactionType::Digital).try_to_vec()?).as_slice(),
            &seller_account.voter_id.to_le_bytes()
        ], 
        bump,
        seeds::program = &orbit_transaction::id()
    )]
    pub seller_transactions_log: Box<Account<'info, SellerOpenTransactions>>,

    #[account(
        mut,
        address = seller_account.wallet
    )]
    pub seller_wallet: Signer<'info>,

    //////////////////////////////////
    /// CPI AND EXTRANEOUS

    #[account(
        seeds = [b"market_authority"],
        bump
    )]
    pub digital_auth: SystemAccount<'info>,

    pub digital_program: Program<'info, OrbitDigitalMarket>,
    
    pub market_account_program: Program<'info, OrbitMarketAccounts>,

    pub transaction_program: Program<'info, OrbitTransaction>
}