use std::convert::TryInto;

use anchor_lang::{
    prelude::*,
    AccountsClose,
    solana_program::{
        program::invoke,
        system_instruction::transfer
    }
};
use pkcs8::DecodePrivateKey;
use rsa::{
    pkcs1::DecodeRsaPublicKey,
    PublicKeyParts
};
use transaction::{transaction_trait::OrbitTransactionTrait, transaction_struct::TransactionState};
use market_accounts::{
    market_account::OrbitMarketAccount, program::OrbitMarketAccounts,
    structs::{
        market_account_trait::OrbitMarketAccountTrait,
        TransactionReviews,
        ReviewErrors,
    },
    MarketAccountErrors
};
use crate::{
    DigitalTransaction,
    DigitalMarketErrors,
    BuyerDecisionState,
    close_escrow_sol,
    close_escrow_spl,

    OpenDigitalTransactionSol,
    CloseDigitalTransactionSol,
    FundEscrowSol,
    
    OpenDigitalTransactionSpl,
    CloseDigitalTransactionSpl,
    FundEscrowSpl,

    post_tx_incrementing,
    submit_rating_with_signer, program::OrbitDigitalMarket
};

////////////////////////////////////////////////////////////////////
/// ORBIT BASE TRANSACTION FUNCTIONALITIES

#[derive(Accounts)]
pub struct CloseTransactionAccount<'info>{
    #[account(
        mut,
        constraint = digital_transaction.metadata.transaction_state == TransactionState::Closed,
    )]
    pub digital_transaction: Box<Account<'info, DigitalTransaction>>,

    #[account(
        constraint = 
            (market_account.key() == digital_transaction.metadata.buyer) ||
            (market_account.key() == digital_transaction.metadata.seller)
    )]
    pub market_account: Account<'info, OrbitMarketAccount>,

    #[account(
        address = market_account.master_pubkey
    )]
    pub account_auth: Signer<'info>,

    #[account(
        mut,
        address = market_account.wallet
    )]
    pub buyer_wallet: SystemAccount<'info>
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g> OrbitTransactionTrait<'a, 'b, 'c, 'd, 'e, 'f, 'g, OpenDigitalTransactionSol<'a>, OpenDigitalTransactionSpl<'b>, CloseDigitalTransactionSol<'c>, CloseDigitalTransactionSpl<'d>, FundEscrowSol<'e>, FundEscrowSpl<'f>, CloseTransactionAccount<'g>> for DigitalTransaction{
    fn open_sol(ctx: Context<OpenDigitalTransactionSol>, price: u64) -> Result<()>{
        ctx.accounts.digital_transaction.metadata.buyer = ctx.accounts.buyer_account.key();
        ctx.accounts.digital_transaction.metadata.seller = ctx.accounts.digital_product.metadata.seller;
        ctx.accounts.digital_transaction.metadata.product = ctx.accounts.digital_product.key();
        ctx.accounts.digital_transaction.metadata.transaction_state = TransactionState::Opened;
        ctx.accounts.digital_transaction.metadata.transaction_price = price;
        ctx.accounts.digital_transaction.metadata.funded = false;
        ctx.accounts.digital_transaction.metadata.currency = ctx.accounts.digital_product.metadata.currency;

        ctx.accounts.digital_transaction.has_comish = false;
        ctx.accounts.digital_transaction.close_rate = 0;

        ctx.accounts.digital_transaction.metadata.escrow_account = ctx.accounts.escrow_account.key();
        ctx.accounts.digital_transaction.product = ctx.accounts.digital_product.key();
        ctx.accounts.digital_transaction.final_decision = BuyerDecisionState::Null;

        ctx.accounts.digital_transaction.reviews = TransactionReviews{
            buyer: false,
            seller: false
        };

        Ok(())
    }

    fn open_spl(ctx: Context<OpenDigitalTransactionSpl>, price: u64) -> Result<()>{
        ctx.accounts.digital_transaction.metadata.buyer = ctx.accounts.buyer_account.key();
        ctx.accounts.digital_transaction.metadata.seller = ctx.accounts.digital_product.metadata.seller;
        ctx.accounts.digital_transaction.metadata.product = ctx.accounts.digital_product.key();
        ctx.accounts.digital_transaction.metadata.transaction_state = TransactionState::Opened;
        ctx.accounts.digital_transaction.metadata.transaction_price = price;
        ctx.accounts.digital_transaction.metadata.funded = false;
        ctx.accounts.digital_transaction.metadata.currency = ctx.accounts.digital_product.metadata.currency;

        ctx.accounts.digital_transaction.has_comish = false;
        ctx.accounts.digital_transaction.close_rate = 0;

        ctx.accounts.digital_transaction.metadata.escrow_account = ctx.accounts.escrow_account.key();
        ctx.accounts.digital_transaction.product = ctx.accounts.digital_product.key();
        ctx.accounts.digital_transaction.final_decision = BuyerDecisionState::Null;

        ctx.accounts.digital_transaction.reviews = TransactionReviews{
            buyer: false,
            seller: false
        };
        Ok(())
    }

    fn close_sol(ctx: Context<CloseDigitalTransactionSol>) -> Result<()>{
        match ctx.bumps.get("escrow_account"){
            Some(escrow_bump) => {
                close_escrow_sol(
                    ctx.accounts.escrow_account.to_account_info(),
                    ctx.accounts.seller_wallet.to_account_info(),
                    &[&[b"orbit_escrow_account", ctx.accounts.digital_transaction.key().as_ref(), &[*escrow_bump]]],
                    ctx.accounts.digital_transaction.close_rate
                ).expect("could not transfer tokens");
                close_escrow_sol(
                    ctx.accounts.escrow_account.to_account_info(),
                    ctx.accounts.buyer_wallet.to_account_info(),
                    &[&[b"orbit_escrow_account", ctx.accounts.digital_transaction.key().as_ref(), &[*escrow_bump]]],
                    100
                ).expect("could not transfer tokens");
            },
            None => return err!(DigitalMarketErrors::InvalidEscrowBump)
        };

        match ctx.bumps.get("digital_auth"){
            Some(auth_bump) => post_tx_incrementing(
                ctx.accounts.market_account_program.to_account_info(),
                ctx.accounts.buyer_account.to_account_info(),
                ctx.accounts.seller_account.to_account_info(),
                ctx.accounts.digital_auth.to_account_info(),
                ctx.accounts.digital_program.to_account_info(),
                &[&[b"market_authority", &[*auth_bump]]]
            ),
            None => return err!(DigitalMarketErrors::InvalidAuthBump)
        }.expect("could not properly invoke market-accounts program");

        ctx.accounts.digital_transaction.metadata.transaction_state = TransactionState::Closed;
        Ok(())
    }

    fn close_spl(ctx: Context<CloseDigitalTransactionSpl>) -> Result<()>{

        match ctx.bumps.get("digital_auth"){
            Some(auth_bump) => {
                close_escrow_spl(
                    ctx.accounts.token_program.to_account_info(),
                    ctx.accounts.escrow_account.to_account_info(),
                    ctx.accounts.seller_token_account.to_account_info(),
                    ctx.accounts.digital_auth.to_account_info(),
                    &[&[b"market_authority", &[*auth_bump]]],
                    ctx.accounts.digital_transaction.metadata.transaction_price,
                    ctx.accounts.digital_transaction.close_rate
                ).expect("could not transfer tokens");
                close_escrow_spl(
                    ctx.accounts.token_program.to_account_info(),
                    ctx.accounts.escrow_account.to_account_info(),
                    ctx.accounts.seller_token_account.to_account_info(),
                    ctx.accounts.digital_auth.to_account_info(),
                    &[&[b"market_authority", &[*auth_bump]]],
                    ctx.accounts.digital_transaction.metadata.transaction_price,
                    100
                ).expect("could not transfer tokens");
                post_tx_incrementing(
                    ctx.accounts.market_account_program.to_account_info(),
                    ctx.accounts.buyer_account.to_account_info(),
                    ctx.accounts.seller_account.to_account_info(),
                    ctx.accounts.digital_auth.to_account_info(),
                    ctx.accounts.digital_program.to_account_info(),
                    &[&[b"market_authority", &[*auth_bump]]]
                )
            },
            None => return err!(DigitalMarketErrors::InvalidAuthBump)
        }.expect("could not properly invoke market-accounts program");

        ctx.accounts.digital_transaction.metadata.transaction_state = TransactionState::Closed;
        Ok(())
    }

    fn fund_escrow_sol(ctx: Context<FundEscrowSol>) -> Result<()>{
        invoke(
            &transfer(
                &ctx.accounts.buyer_wallet.key(),
                &ctx.accounts.escrow_account.key(),
                ctx.accounts.digital_transaction.metadata.transaction_price
            ),
            &[
                ctx.accounts.buyer_wallet.to_account_info(),
                ctx.accounts.escrow_account.to_account_info()
            ]
        ).expect("could not fund escrow");
        ctx.accounts.digital_transaction.metadata.funded = true;
        ctx.accounts.digital_transaction.metadata.transaction_state = TransactionState::BuyerFunded;
        Ok(())
    }

    fn fund_escrow_spl(ctx: Context<FundEscrowSpl>) -> Result<()>{
        anchor_spl::token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(), 
                anchor_spl::token::Transfer{
                    from: ctx.accounts.buyer_spl_wallet.to_account_info(),
                    to: ctx.accounts.escrow_account.to_account_info(),
                    authority: ctx.accounts.wallet_owner.to_account_info()
                }
            ),
            ctx.accounts.digital_transaction.metadata.transaction_price
        ).expect("could not fund escrow account. maybe check your balance");
        ctx.accounts.digital_transaction.metadata.funded = true;
        ctx.accounts.digital_transaction.metadata.transaction_state = TransactionState::BuyerFunded;
        Ok(())
    }

    fn close_transaction_account(ctx: Context<CloseTransactionAccount>) -> Result<()>{
        ctx.accounts.digital_transaction.close(ctx.accounts.market_account.to_account_info())
    }
}

//////////////////////////////////////////////////////////////////////////
/// BUYER CONFIRMATIONS

#[derive(Accounts)]
pub struct BuyerConfirmation<'info>{
    #[account(
        mut,
        constraint = digital_transaction.metadata.buyer == buyer_account.key()
    )]
    pub digital_transaction: Box<Account<'info, DigitalTransaction>>,

    #[account(
        has_one = master_pubkey
    )]
    pub buyer_account: Account<'info, OrbitMarketAccount>,

    pub master_pubkey: Signer<'info>,
}

pub fn confirm_delivered_handler(ctx: Context<BuyerConfirmation>) -> Result<()>{
    if ctx.accounts.digital_transaction.metadata.transaction_state != TransactionState::Shipped{
        return err!(DigitalMarketErrors::WaitingForSellerData);
    }
    ctx.accounts.digital_transaction.metadata.transaction_state = TransactionState::BuyerConfirmedDelivery;

    Ok(())
}

pub fn confirm_accept_handler(ctx: Context<BuyerConfirmation>) -> Result<()>{
    if ctx.accounts.digital_transaction.metadata.transaction_state != TransactionState::BuyerConfirmedDelivery{
        return err!(DigitalMarketErrors::DidNotConfirmDelivery);
    }
    ctx.accounts.digital_transaction.close_rate = 100;
    ctx.accounts.digital_transaction.metadata.transaction_state = TransactionState::BuyerConfirmedProduct;

    Ok(())
}

pub fn deny_accept_handler(ctx: Context<BuyerConfirmation>) -> Result<()>{
    if ctx.accounts.digital_transaction.metadata.transaction_state != TransactionState::BuyerConfirmedDelivery{
        return err!(DigitalMarketErrors::DidNotConfirmDelivery);
    }
    ctx.accounts.digital_transaction.metadata.transaction_state = TransactionState::BuyerConfirmedProduct;

    Ok(())
}

///////////////////////////////////////////////////////////////////////
/// SELLER CONFIRMATIONS

#[derive(Accounts)]
pub struct SellerAcceptTransaction<'info>{
    #[account(
        mut,
        constraint = digital_transaction.metadata.transaction_state == TransactionState::Opened
    )]
    pub digital_transaction: Box<Account<'info, DigitalTransaction>>,

    #[account(
        address = digital_transaction.metadata.seller,
        has_one = master_pubkey
    )]
    pub seller_account: Account<'info, OrbitMarketAccount>,

    pub master_pubkey: Signer<'info>
}

pub fn seller_accept_transaction_handler(ctx: Context<SellerAcceptTransaction>) -> Result<()>{
    ctx.accounts.digital_transaction.metadata.transaction_state = TransactionState::SellerConfirmed;
    Ok(())
}

#[derive(Accounts)]
pub struct CommitInitData<'info>{
    #[account(
        mut,
        constraint = digital_transaction.metadata.transaction_state == TransactionState::SellerConfirmed
    )]
    pub digital_transaction: Box<Account<'info, DigitalTransaction>>,

    #[account(
        address = digital_transaction.metadata.seller
    )]
    pub seller_account: Account<'info, OrbitMarketAccount>,

    #[account(
        address = seller_account.master_pubkey
    )]
    pub seller_auth: Signer<'info>,
}

pub fn commit_init_keys_handler(ctx: Context<CommitInitData>, pem: String, index: u8) -> Result<()>{   
    let rsa_pubkey = rsa::RsaPublicKey::from_pkcs1_pem(&pem).expect("could not decode public key");
    let modulo = rsa_pubkey.n();
    let exp = rsa_pubkey.e();
    ctx.accounts.digital_transaction.rsa_pubkeys_array[index as usize] = [modulo.to_bytes_le().try_into().expect(""), exp.to_bytes_le().try_into().expect("")];
    Ok(())
}

pub fn commit_link_handler(ctx: Context<CommitInitData>, link: [u8; 64]) -> Result<()>{
    ctx.accounts.digital_transaction.data_address = link;
    Ok(())
}

pub fn update_status_to_shipping_handler(ctx: Context<CommitInitData>) -> Result<()>{
    ctx.accounts.digital_transaction.metadata.transaction_state = TransactionState::Shipped;
    Ok(())
}

#[derive(Accounts)]
pub struct CommitSubKeys<'info>{
    #[account(
        mut,
        constraint = digital_transaction.metadata.transaction_state == TransactionState::SellerConfirmed
    )]
    pub digital_transaction: Box<Account<'info, DigitalTransaction>>,

    #[account(
        address = digital_transaction.metadata.seller
    )]
    pub seller_account: Account<'info, OrbitMarketAccount>,

    #[account(
        address = seller_account.master_pubkey
    )]
    pub seller_auth: Signer<'info>,
}

pub fn commit_subkeys_handler(ctx: Context<CommitSubKeys>, index: u8, pem: String) -> Result<()>{
    let rsa_priv_key = rsa::RsaPrivateKey::from_pkcs8_pem(&pem).expect("could not decode private key");
    let rsa_pub_key = rsa::RsaPublicKey::from(rsa_priv_key);

    if  (rsa::BigUint::from_bytes_le(&ctx.accounts.digital_transaction.rsa_pubkeys_array[index as usize][0]) != *rsa_pub_key.n()) ||
        (rsa::BigUint::from_bytes_le(&ctx.accounts.digital_transaction.rsa_pubkeys_array[index as usize][1]) != *rsa_pub_key.e())
    {
        return err!(DigitalMarketErrors::IncorrectPrivateKey)
    }
    Ok(())
}

///////////////////////////////////////////////////////////////////////
/// ACCOUNT HELPERS (leave a review!)

#[derive(Accounts)]
pub struct LeaveReview<'info>{
    #[account(mut)]
    pub digital_transaction: Box<Account<'info, DigitalTransaction>>,

    #[account(
        mut,
        constraint = 
        (reviewer.key() == digital_transaction.metadata.seller) ||
        (reviewer.key() == digital_transaction.metadata.buyer)
    )]
    pub reviewed_account: Account<'info, OrbitMarketAccount>,

    #[account(
        constraint = 
        (reviewer.key() == digital_transaction.metadata.seller) ||
        (reviewer.key() == digital_transaction.metadata.buyer),
        has_one = master_pubkey
    )]
    pub reviewer: Account<'info, OrbitMarketAccount>,

    pub master_pubkey: Signer<'info>,

    pub accounts_program: Program<'info, OrbitMarketAccounts>,

    #[account(
        seeds = [b"market_authority"],
        bump
    )]
    pub digital_auth: SystemAccount<'info>,

    pub digital_program: Program<'info, OrbitDigitalMarket>
}

impl <'a> OrbitMarketAccountTrait<'a, LeaveReview<'a>> for DigitalTransaction{
    fn leave_review(ctx: Context<LeaveReview>, rating: u8) -> Result<()>{
        if ctx.accounts.reviewer.key() == ctx.accounts.reviewed_account.key(){
            return err!(ReviewErrors::InvalidReviewAuthority)
        };
        if rating == 0 || rating > 5{
            return err!(ReviewErrors::RatingOutsideRange)
        };

        if ctx.accounts.digital_transaction.metadata.seller == ctx.accounts.reviewer.key() && !ctx.accounts.digital_transaction.reviews.seller{
            match ctx.bumps.get("digital_auth"){
                Some(auth_bump) => {
                    submit_rating_with_signer(
                        ctx.accounts.accounts_program.to_account_info(),
                        ctx.accounts.reviewed_account.to_account_info(),
                        ctx.accounts.digital_auth.to_account_info(),
                        ctx.accounts.digital_program.to_account_info(),
                        &[&[b"market_authority", &[*auth_bump]]],
                        rating
                    );
                    ctx.accounts.digital_transaction.reviews.seller = true;
                },
                None => return err!(MarketAccountErrors::CannotCallOrbitAccountsProgram)
            }
        }else
        if ctx.accounts.digital_transaction.metadata.buyer == ctx.accounts.reviewer.key()  && !ctx.accounts.digital_transaction.reviews.buyer{
            match ctx.bumps.get("digital_auth"){
                Some(auth_bump) => {
                    submit_rating_with_signer(
                        ctx.accounts.accounts_program.to_account_info(),
                        ctx.accounts.reviewed_account.to_account_info(),
                        ctx.accounts.digital_auth.to_account_info(),
                        ctx.accounts.digital_program.to_account_info(),
                        &[&[b"market_authority", &[*auth_bump]]],
                        rating
                    );
                    ctx.accounts.digital_transaction.reviews.buyer = true;
                },
                None => return err!(MarketAccountErrors::CannotCallOrbitAccountsProgram)
            }
            ctx.accounts.digital_transaction.reviews.buyer = true;
        }else
        {
            return err!(ReviewErrors::InvalidReviewAuthority)
        };

        Ok(())
    }

}