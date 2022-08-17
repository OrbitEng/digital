use std::convert::TryInto;
use anchor_lang::{
    prelude::*,
    AccountsClose,
    solana_program::{
        program::invoke,
        system_instruction::transfer
    }
};
use transaction::{transaction_trait::OrbitTransactionTrait, transaction_struct::TransactionState};
use market_accounts::{market_account::OrbitMarketAccount, program::OrbitMarketAccounts, cpi::accounts::IncrementTransactions};
use crate::{DigitalTransaction, DigitalProduct, DigitalMarketErrors, close_escrow, BuyerDecisionState};

////////////////////////////////////////////////////////////////////
/// ORBIT BASE TRANSACTION FUNCTIONALITIES
#[derive(Accounts)]
pub struct OpenDigitalTransaction<'info>{
    #[account(
        init,
        space = 2000,
        payer = buyer_wallet,
    )]
    pub digital_transaction: Box<Account<'info, DigitalTransaction>>,

    pub digital_product: Account<'info, DigitalProduct>,

    #[account(
        constraint = buyer_account.wallet == buyer_wallet.key()
    )]
    pub buyer_account: Account<'info, OrbitMarketAccount>,

    #[account(mut)]
    pub buyer_wallet: Signer<'info>,

    #[account(
        seeds = [
            b"orbit_escrow_account",
            digital_transaction.key().as_ref()
        ],
        bump
    )]
    pub escrow_account: SystemAccount<'info>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct CloseDigitalTransaction<'info>{
    #[account(
        mut,
        constraint = digital_transaction.metadata.transaction_state == TransactionState::BuyerConfirmedProduct,
        constraint = digital_transaction.final_decision != BuyerDecisionState::Null,
        constraint = digital_transaction.metadata.escrow_account == escrow_account.key()
    )]
    pub digital_transaction: Box<Account<'info, DigitalTransaction>>,

    #[account(
        seeds = [
            b"orbit_escrow_account",
            digital_transaction.key().as_ref()
        ],
        bump
    )]
    pub escrow_account: SystemAccount<'info>,

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
        address = digital_transaction.metadata.buyer
    )]
    pub buyer_account: Account<'info, OrbitMarketAccount>,

    #[account(
        mut,
        address = buyer_account.wallet
    )]
    pub buyer_wallet: SystemAccount<'info>,

    #[account(
        seeds = [
            b"digital_auth"
        ],
        bump
    )]
    pub digital_auth: SystemAccount<'info>,

    #[account(
        address = market_accounts::ID
    )]
    pub market_account_program: Program<'info, OrbitMarketAccounts>
}

#[derive(Accounts)]
pub struct FundEscrow<'info>{
    #[account(
        mut,
        constraint = digital_transaction.metadata.transaction_state == TransactionState::SellerConfirmed,
        constraint = digital_transaction.metadata.escrow_account == escrow_account.key()
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
        bump
    )]
    pub escrow_account: SystemAccount<'info>,

    #[account(
        mut,
        address = buyer_account.wallet
    )]
    pub buyer_wallet: Signer<'info>
}

#[derive(Accounts)]
pub struct CloseTransactionAccount<'info>{
    #[account(
        mut,
        constraint = digital_transaction.metadata.transaction_state == TransactionState::Closed,
    )]
    pub digital_transaction: Account<'info, DigitalTransaction>,

    #[account(
        constraint = 
            (market_account.key() == digital_transaction.metadata.buyer) ||
            (market_account.key() == digital_transaction.metadata.seller)
    )]
    pub market_account: Account<'info, OrbitMarketAccount>,

    #[account(
        mut,
        address = market_account.wallet
    )]
    pub buyer_wallet: SystemAccount<'info>
}

impl<'a, 'b, 'c, 'd> OrbitTransactionTrait<'a, 'b, 'c, 'd,  OpenDigitalTransaction<'a>, CloseDigitalTransaction<'b>, FundEscrow<'c>, CloseTransactionAccount<'d>> for DigitalTransaction{
    fn open(ctx: Context<OpenDigitalTransaction>, price: u64) -> Result<()>{
        
        ctx.accounts.digital_transaction.metadata.buyer = ctx.accounts.buyer_account.key();
        ctx.accounts.digital_transaction.metadata.seller = ctx.accounts.digital_product.metadata.seller;
        ctx.accounts.digital_transaction.metadata.product = ctx.accounts.digital_product.key();
        ctx.accounts.digital_transaction.metadata.transaction_state = TransactionState::Opened;
        ctx.accounts.digital_transaction.metadata.transaction_price = price;
        ctx.accounts.digital_transaction.metadata.funded = false;

        ctx.accounts.digital_transaction.has_comish = false;
        ctx.accounts.digital_transaction.close_rate = 0;

        ctx.accounts.digital_transaction.metadata.escrow_account = ctx.accounts.escrow_account.key();
        ctx.accounts.digital_transaction.product = ctx.accounts.digital_product.key();
        ctx.accounts.digital_transaction.final_decision = BuyerDecisionState::Null;
        Ok(())
    }

    fn close(ctx: Context<CloseDigitalTransaction>) -> Result<()>{
        match ctx.bumps.get("escrow_account"){
            Some(escrow_bump) => {
                match close_escrow(
                    ctx.accounts.escrow_account.to_account_info(),
                    ctx.accounts.seller_wallet.to_account_info(),
                    &[&[b"orbit_escrow_account", ctx.accounts.digital_transaction.key().as_ref(), &[*escrow_bump]]],
                    ctx.accounts.digital_transaction.close_rate
                ){
                    Ok(_) => close_escrow(
                        ctx.accounts.escrow_account.to_account_info(),
                        ctx.accounts.buyer_wallet.to_account_info(),
                        &[&[b"orbit_escrow_account", ctx.accounts.digital_transaction.key().as_ref(), &[*escrow_bump]]],
                        100
                    ),
                    Err(e) => Err(e)
                }
            },
            None => return err!(DigitalMarketErrors::InvalidEscrowBump)
        }.expect("could not properly close");

        match ctx.bumps.get("digital_auth"){
            Some(auth_bump) => {
                market_accounts::cpi::post_tx(
                    CpiContext::new_with_signer(
                        ctx.accounts.market_account_program.to_account_info(),
                        IncrementTransactions{
                            market_account: ctx.accounts.buyer_account.to_account_info(),
                            invoker: ctx.accounts.digital_auth.to_account_info()
                        },
                        &[&[b"digital_auth", &[*auth_bump]]])
                ).expect("could not properly invoke market-accounts program");
                market_accounts::cpi::post_tx(
                    CpiContext::new_with_signer(
                        ctx.accounts.market_account_program.to_account_info(),
                        IncrementTransactions{
                            market_account: ctx.accounts.seller_account.to_account_info(),
                            invoker: ctx.accounts.digital_auth.to_account_info()
                        },
                        &[&[b"digital_auth", &[*auth_bump]]])
                )
            },
            None => return err!(DigitalMarketErrors::InvalidAuthBump)
        }.expect("could not properly invoke market-accounts program");

        // todo: think what to do instead of immediately closing. we dont want to store transactions forever!
        ctx.accounts.digital_transaction.metadata.transaction_state = TransactionState::Closed;
        Ok(())
    }

    fn fund_escrow(ctx: Context<FundEscrow>) -> Result<()>{
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
        address = digital_transaction.product
    )]
    pub digital_product: Account<'info, DigitalProduct>,

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
pub struct CommitKeys<'info>{
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

pub fn commit_keys_handler(ctx: Context<CommitKeys>, keys: Vec<u8>, link: [u8; 64]) -> Result<()>{
    ctx.accounts.digital_transaction.metadata.transaction_state = TransactionState::Shipped;
    let mut key_arr = [[0 as u8; 16]; 64];
    // let mut key_arr = [[0 as u8; 16]; 64];
    for i in 0..(keys.len()/16){
        key_arr[i] = keys[(16*i) as usize..((16*i)+15) as usize].try_into().expect("wrong key size");
    };
    ctx.accounts.digital_transaction.keys_array = key_arr;
    ctx.accounts.digital_transaction.data_address = link;
    Ok(())
}

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