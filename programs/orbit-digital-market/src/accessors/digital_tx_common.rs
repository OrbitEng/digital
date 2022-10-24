use anchor_lang::{
    prelude::*,
    AccountsClose,
    solana_program::{
        program::{
            invoke,
            invoke_signed
        },
        system_instruction::transfer
    }
};
use orbit_transaction::{
    transaction_trait::OrbitTransactionTrait,
    transaction_struct::TransactionState,
    TransactionReviews,
    BuyerOpenTransactions,
    TransactionErrors, SellerOpenTransactions
};
use market_accounts::{
    market_account::OrbitMarketAccount, program::OrbitMarketAccounts,
    structs::market_account_trait::OrbitMarketAccountTrait,
    ReviewErrors,
    MarketAccountErrors,
    OrbitReflink
};
use crate::{
    DigitalTransaction,
    DigitalMarketErrors,
    BuyerDecisionState,

    OpenDigitalTransactionSol,
    CloseDigitalTransactionSol,
    
    OpenDigitalTransactionSpl,
    CloseDigitalTransactionSpl,
    FundEscrowSol,
    FundEscrowSpl,
    
    program::OrbitDigitalMarket, id, SellerEarlyDeclineSol, SellerEarlyDeclineSpl
};
use anchor_spl::token::{
    accessor::amount,
    TokenAccount
};

////////////////////////////////////////////////////////////////////
/// ORBIT BASE TRANSACTION FUNCTIONALITIES

#[derive(Accounts)]
pub struct CloseTransactionAccount<'info>{
    #[account(
        mut,
        constraint = digital_transaction.metadata.transaction_state == TransactionState::Closed,
    )]
    pub digital_transaction: Account<'info, DigitalTransaction>,

    #[account(
        constraint = 
            (transactions_log.key() == digital_transaction.metadata.buyer) ||
            (transactions_log.key() == digital_transaction.metadata.seller),
        constraint = transactions_log.try_borrow_data()?[8..40] == wallet.key().to_bytes()
    )]
    /// CHECK: basic checks
    pub transactions_log: AccountInfo<'info>,

    pub wallet: Signer<'info>,

    #[account(
        mut
    )]
    pub buyer_wallet: SystemAccount<'info>
}


impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i> OrbitTransactionTrait<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, OpenDigitalTransactionSol<'a>, OpenDigitalTransactionSpl<'b>, CloseDigitalTransactionSol<'c>, CloseDigitalTransactionSpl<'d>, FundEscrowSol<'e>, FundEscrowSpl<'f>, CloseTransactionAccount<'g>, SellerEarlyDeclineSol<'h>, SellerEarlyDeclineSpl<'i>> for DigitalTransaction{
    fn open_sol(ctx: Context<OpenDigitalTransactionSol>, seller_index: u8, buyer_index: u8, mut price: u64, use_discount: bool) -> Result<()>{
        let auth_bump: &u8;
        if let Some(ab) = ctx.bumps.get("digital_auth"){
            auth_bump = ab
        }else{
            return err!(DigitalMarketErrors::InvalidAuthBump)
        };
        if use_discount && ctx.accounts.buyer_market_account.dispute_discounts > 0{
            ctx.accounts.digital_transaction.metadata.rate = 100;
            price = price * 95 / 100;
            market_accounts::cpi::decrement_dispute_discounts(
                CpiContext::new_with_signer(
                    ctx.accounts.market_account_program.to_account_info(),
                    market_accounts::cpi::accounts::MarketAccountUpdateInternal{
                        market_account: ctx.accounts.buyer_market_account.to_account_info(),
                        caller_auth: ctx.accounts.digital_auth.to_account_info(),
                        caller: ctx.accounts.digital_program.to_account_info()
                    },
                    &[&[b"market_auth", &[*auth_bump]]]
                )
            )?;
        }else{
            ctx.accounts.digital_transaction.metadata.rate = 95
        }
        ctx.accounts.digital_transaction.metadata.buyer = ctx.accounts.buyer_transactions_log.key();
        ctx.accounts.digital_transaction.metadata.seller = ctx.accounts.seller_transactions_log.key();
        ctx.accounts.digital_transaction.metadata.product = ctx.accounts.digital_product.key();
        ctx.accounts.digital_transaction.metadata.transaction_state = TransactionState::Opened;
        ctx.accounts.digital_transaction.metadata.transaction_price = price;
        ctx.accounts.digital_transaction.metadata.funded = false;
        ctx.accounts.digital_transaction.metadata.currency = System::id();

        ctx.accounts.digital_transaction.num_keys = 0;

        ctx.accounts.digital_transaction.metadata.escrow_account = ctx.accounts.escrow_account.key();
        ctx.accounts.digital_transaction.final_decision = BuyerDecisionState::Null;

        ctx.accounts.digital_transaction.metadata.reviews = TransactionReviews{
            buyer: false,
            seller: false
        };

        orbit_transaction::cpi::add_to_buyer_transactions_log(
            CpiContext::new(
                ctx.accounts.transaction_program.to_account_info(),
                orbit_transaction::cpi::accounts::AddBuyerTransactions{
                    transactions_log: ctx.accounts.buyer_transactions_log.to_account_info(),
                    tx: ctx.accounts.digital_transaction.to_account_info(),
                    buyer_wallet: ctx.accounts.buyer_wallet.to_account_info()
                }
            ),
            buyer_index
        )?;
        orbit_transaction::cpi::add_to_seller_transactions_log(
            CpiContext::new_with_signer(
                ctx.accounts.transaction_program.to_account_info(),
                orbit_transaction::cpi::accounts::AddSellerTransactions{
                    transactions_log: ctx.accounts.seller_transactions_log.to_account_info(),
                    tx: ctx.accounts.digital_transaction.to_account_info(),
                    caller_auth: ctx.accounts.digital_auth.to_account_info(),
                    caller: ctx.accounts.digital_program.to_account_info()
                },
                &[&[b"market_auth", &[*auth_bump]]]
            ),
            seller_index
        )?;

        Ok(())
    }

    fn open_spl(ctx: Context<OpenDigitalTransactionSpl>, seller_index: u8, buyer_index: u8, mut price: u64, use_discount: bool) -> Result<()>{
        let auth_bump: &u8;
        if let Some(ab) = ctx.bumps.get("digital_auth"){
            auth_bump = ab
        }else{
            return err!(DigitalMarketErrors::InvalidAuthBump)
        };
        if use_discount && ctx.accounts.buyer_market_account.dispute_discounts > 0{
            ctx.accounts.digital_transaction.metadata.rate = 100;
            price = price * 95 / 100;
            market_accounts::cpi::decrement_dispute_discounts(
                CpiContext::new_with_signer(
                    ctx.accounts.market_account_program.to_account_info(),
                    market_accounts::cpi::accounts::MarketAccountUpdateInternal{
                        market_account: ctx.accounts.buyer_market_account.to_account_info(),
                        caller_auth: ctx.accounts.digital_auth.to_account_info(),
                        caller: ctx.accounts.digital_program.to_account_info()
                    },
                    &[&[b"market_auth", &[*auth_bump]]]
                )
            )?;
        }else{
            ctx.accounts.digital_transaction.metadata.rate = 95
        }
        ctx.accounts.digital_transaction.metadata.buyer = ctx.accounts.buyer_transactions_log.key();
        ctx.accounts.digital_transaction.metadata.seller = ctx.accounts.seller_transactions_log.key();
        ctx.accounts.digital_transaction.metadata.product = ctx.accounts.digital_product.key();
        ctx.accounts.digital_transaction.metadata.transaction_state = TransactionState::Opened;
        ctx.accounts.digital_transaction.metadata.transaction_price = price;
        ctx.accounts.digital_transaction.metadata.funded = false;
        ctx.accounts.digital_transaction.metadata.currency = ctx.accounts.token_mint.key();
        
        ctx.accounts.digital_transaction.num_keys = 0;

        ctx.accounts.digital_transaction.metadata.escrow_account = ctx.accounts.escrow_account.key();
        ctx.accounts.digital_transaction.final_decision = BuyerDecisionState::Null;

        ctx.accounts.digital_transaction.metadata.reviews = TransactionReviews{
            buyer: false,
            seller: false
        };

        orbit_transaction::cpi::add_to_buyer_transactions_log(
            CpiContext::new(
                ctx.accounts.transaction_program.to_account_info(),
                orbit_transaction::cpi::accounts::AddBuyerTransactions{
                    transactions_log: ctx.accounts.buyer_transactions_log.to_account_info(),
                    tx: ctx.accounts.digital_transaction.to_account_info(),
                    buyer_wallet: ctx.accounts.buyer_wallet.to_account_info()
                }
            ),
            buyer_index
        )?;
        orbit_transaction::cpi::add_to_seller_transactions_log(
            CpiContext::new_with_signer(
                ctx.accounts.transaction_program.to_account_info(),
                orbit_transaction::cpi::accounts::AddSellerTransactions{
                    transactions_log: ctx.accounts.seller_transactions_log.to_account_info(),
                    tx: ctx.accounts.digital_transaction.to_account_info(),
                    caller_auth: ctx.accounts.digital_auth.to_account_info(),
                    caller: ctx.accounts.digital_program.to_account_info()
                },
                &[&[b"market_auth", &[*auth_bump]]]
            ),
            seller_index
        )?;
        Ok(())
    }

    fn close_sol(ctx: Context<'_, '_, '_, 'c, CloseDigitalTransactionSol<'c>>) -> Result<()>{
        match ctx.bumps.get("escrow_account"){
            Some(escrow_bump) => {
                if (ctx.accounts.digital_transaction.metadata.rate == 95)
                && (ctx.accounts.digital_transaction.final_decision == BuyerDecisionState::Accept){
                    let bal = ctx.accounts.escrow_account.lamports();
                    let mut residual_amt = bal * 5/100;
                    if  (ctx.accounts.buyer_account.used_reflink != Pubkey::new(&[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0])) &&
                        (ctx.remaining_accounts[0].key() == ctx.accounts.buyer_account.used_reflink)
                    {
                        let reflink_amt = bal * 25 / 10000;
                        residual_amt = bal * 45/1000;
                        orbit_transaction::close_escrow_sol_flat!(
                            ctx.accounts.escrow_account.to_account_info(),
                            ctx.accounts.buyer_wallet.to_account_info(),
                            &[&[b"orbit_escrow_account", ctx.accounts.digital_transaction.key().as_ref(), &[*escrow_bump]]],
                            reflink_amt
                        ).expect("couldnt close escrow");
                        match orbit_transaction::remaining_accounts_to_wallet!(ctx.remaining_accounts){
                            Ok(reflink_wallet) => {
                                orbit_transaction::close_escrow_sol_flat!(
                                    ctx.accounts.escrow_account.to_account_info(),
                                    reflink_wallet.to_account_info(),
                                    &[&[b"orbit_escrow_account", ctx.accounts.digital_transaction.key().as_ref(), &[*escrow_bump]]],
                                    reflink_amt
                                ).expect("couldnt close escrow");
                                reflink_wallet.exit(ctx.program_id)?;
                            },
                            Err(e) => return Err(e)
                        }
                    }
                    orbit_transaction::close_escrow_sol_flat!(
                        ctx.accounts.escrow_account.to_account_info(),
                        ctx.accounts.multisig_wallet.to_account_info(),
                        &[&[b"orbit_escrow_account", ctx.accounts.digital_transaction.key().as_ref(), &[*escrow_bump]]],
                        residual_amt
                    ).expect("couldnt close escrow");
                }

                orbit_transaction::close_escrow_sol_rate!(
                    ctx.accounts.escrow_account.to_account_info(),
                    ctx.accounts.seller_wallet.to_account_info(),
                    &[&[b"orbit_escrow_account", ctx.accounts.digital_transaction.key().as_ref(), &[*escrow_bump]]],
                    ctx.accounts.digital_transaction.metadata.rate
                ).expect("could not transfer tokens");
                orbit_transaction::close_escrow_sol_rate!(
                    ctx.accounts.escrow_account.to_account_info(),
                    ctx.accounts.buyer_wallet.to_account_info(),
                    &[&[b"orbit_escrow_account", ctx.accounts.digital_transaction.key().as_ref(), &[*escrow_bump]]],
                    100
                ).expect("could not transfer tokens");
            },
            None => return err!(DigitalMarketErrors::InvalidEscrowBump)
        };

        if let Some(auth_bump) = ctx.bumps.get("digital_auth"){
            orbit_transaction::post_tx_incrementing!(
                ctx.accounts.market_account_program.to_account_info(),
                ctx.accounts.buyer_account.to_account_info(),
                ctx.accounts.seller_account.to_account_info(),
                ctx.accounts.digital_auth.to_account_info(),
                ctx.accounts.digital_program.to_account_info(),
                &[&[b"market_authority", &[*auth_bump]]]
            )
        }else{
            return err!(DigitalMarketErrors::InvalidAuthBump)
        }?;

        orbit_transaction::cpi::clear_from_seller_transactions_log(
            CpiContext::new(
                ctx.accounts.transaction_program.to_account_info(),
                orbit_transaction::cpi::accounts::ClearSellerTransactions{
                    transactions_log: ctx.accounts.seller_transactions_log.to_account_info(),
                    caller_auth: ctx.accounts.digital_auth.to_account_info(),
                    caller: ctx.accounts.digital_program.to_account_info()
                }
            ),
            ctx.accounts.digital_transaction.metadata.seller_tx_index
        )?;

        orbit_transaction::cpi::clear_from_buyer_transactions_log(
            CpiContext::new(
                ctx.accounts.transaction_program.to_account_info(),
                orbit_transaction::cpi::accounts::ClearBuyerTransactions{
                    transactions_log: ctx.accounts.buyer_transactions_log.to_account_info(),
                    caller_auth: ctx.accounts.digital_auth.to_account_info(),
                    caller: ctx.accounts.digital_program.to_account_info()
                }
            ),
            ctx.accounts.digital_transaction.metadata.seller_tx_index
        )?;

        ctx.accounts.digital_transaction.metadata.transaction_state = TransactionState::Closed;
        Ok(())
    }

    fn close_spl(ctx: Context<'_, '_, '_, 'd, CloseDigitalTransactionSpl<'d>>) -> Result<()>{
        if let Some(auth_bump) = ctx.bumps.get("digital_auth"){
            if (ctx.accounts.digital_transaction.metadata.rate == 95)
                && (ctx.accounts.digital_transaction.final_decision == BuyerDecisionState::Accept){
                    let bal = amount(&ctx.accounts.escrow_account.to_account_info()).expect("could not deserialize token account");
                    let mut residual_amt = bal * 5/100;
                    if  (ctx.accounts.buyer_account.used_reflink != Pubkey::new(&[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0])) &&
                        (ctx.remaining_accounts[0].key() == ctx.accounts.buyer_account.used_reflink)
                    {
                        let reflink_amt = bal * 25 / 10000;
                        residual_amt = bal * 45/1000;
                        orbit_transaction::close_escrow_spl_flat!(
                            ctx.accounts.token_program.to_account_info(),
                            ctx.accounts.escrow_account.to_account_info(),
                            ctx.accounts.buyer_token_account.to_account_info(),
                            ctx.accounts.digital_auth.to_account_info(),
                            &[&[b"market_authority", &[*auth_bump]]],
                            reflink_amt
                        ).expect("couldnt close escrow");
                        

                        match orbit_transaction::remaining_accounts_to_token_account!(ctx.remaining_accounts){
                            Ok(reflink_token_account) => {
                                orbit_transaction::close_escrow_spl_flat!(
                                    ctx.accounts.token_program.to_account_info(),
                                    ctx.accounts.escrow_account.to_account_info(),
                                    reflink_token_account.to_account_info(),
                                    ctx.accounts.digital_auth.to_account_info(),
                                    &[&[b"market_authority", &[*auth_bump]]],
                                    reflink_amt
                                ).expect("couldnt close escrow");
                                reflink_token_account.exit(ctx.program_id)?;
                            },
                            Err(e) => return Err(e)
                        }
                    }
                    orbit_transaction::close_escrow_spl_flat!(
                        ctx.accounts.token_program.to_account_info(),
                        ctx.accounts.escrow_account.to_account_info(),
                        ctx.accounts.multisig_ata.to_account_info(),
                        ctx.accounts.digital_auth.to_account_info(),
                        &[&[b"market_authority", &[*auth_bump]]],
                        residual_amt
                    ).expect("couldnt close escrow");
                }

                orbit_transaction::close_escrow_spl_rate!(
                    ctx.accounts.token_program.to_account_info(),
                    ctx.accounts.escrow_account.to_account_info(),
                    ctx.accounts.seller_token_account.to_account_info(),
                    ctx.accounts.digital_auth.to_account_info(),
                    &[&[b"market_authority", &[*auth_bump]]],
                    ctx.accounts.digital_transaction.metadata.transaction_price,
                    ctx.accounts.digital_transaction.metadata.rate
                ).expect("could not transfer tokens");
                orbit_transaction::close_escrow_spl_rate!(
                    ctx.accounts.token_program.to_account_info(),
                    ctx.accounts.escrow_account.to_account_info(),
                    ctx.accounts.buyer_token_account.to_account_info(),
                    ctx.accounts.digital_auth.to_account_info(),
                    &[&[b"market_authority", &[*auth_bump]]],
                    ctx.accounts.digital_transaction.metadata.transaction_price,
                    100
                ).expect("could not transfer tokens");
                orbit_transaction::post_tx_incrementing!(
                    ctx.accounts.market_account_program.to_account_info(),
                    ctx.accounts.buyer_account.to_account_info(),
                    ctx.accounts.seller_account.to_account_info(),
                    ctx.accounts.digital_auth.to_account_info(),
                    ctx.accounts.digital_program.to_account_info(),
                    &[&[b"market_authority", &[*auth_bump]]]
                )
        }else{
            return err!(DigitalMarketErrors::InvalidAuthBump)
        }?;

        orbit_transaction::cpi::clear_from_seller_transactions_log(
            CpiContext::new(
                ctx.accounts.transaction_program.to_account_info(),
                orbit_transaction::cpi::accounts::ClearSellerTransactions{
                    transactions_log: ctx.accounts.seller_transactions_log.to_account_info(),
                    caller_auth: ctx.accounts.digital_auth.to_account_info(),
                    caller: ctx.accounts.digital_program.to_account_info()
                }
            ),
            ctx.accounts.digital_transaction.metadata.seller_tx_index
        )?;

        orbit_transaction::cpi::clear_from_buyer_transactions_log(
            CpiContext::new(
                ctx.accounts.transaction_program.to_account_info(),
                orbit_transaction::cpi::accounts::ClearBuyerTransactions{
                    transactions_log: ctx.accounts.buyer_transactions_log.to_account_info(),
                    caller_auth: ctx.accounts.digital_auth.to_account_info(),
                    caller: ctx.accounts.digital_program.to_account_info()
                }
            ),
            ctx.accounts.digital_transaction.metadata.seller_tx_index
        )?;

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
                    from: ctx.accounts.buyer_token_account.to_account_info(),
                    to: ctx.accounts.escrow_account.to_account_info(),
                    authority: ctx.accounts.buyer_wallet.to_account_info()
                }
            ),
            ctx.accounts.digital_transaction.metadata.transaction_price
        ).expect("could not fund escrow account. maybe check your balance");
        ctx.accounts.digital_transaction.metadata.funded = true;
        ctx.accounts.digital_transaction.metadata.transaction_state = TransactionState::BuyerFunded;
        Ok(())
    }

    fn close_transaction_account(ctx: Context<CloseTransactionAccount>) -> Result<()>{
        ctx.accounts.digital_transaction.close(ctx.accounts.buyer_wallet.to_account_info())
    }

    fn seller_early_decline_sol(ctx: Context<SellerEarlyDeclineSol>) -> Result<()>{
        ctx.accounts.digital_transaction.metadata.transaction_state = TransactionState::Closed;
        if ctx.accounts.digital_transaction.metadata.rate == 100{
            market_accounts::cpi::increment_dispute_discounts(
                CpiContext::new(
                    ctx.accounts.market_account_program.to_account_info(),
                    market_accounts::cpi::accounts::MarketAccountUpdateInternal{
                        market_account: ctx.accounts.buyer_account.to_account_info(),
                        caller_auth: ctx.accounts.digital_auth.to_account_info(),
                        caller: ctx.accounts.digital_program.to_account_info()
                    }
                )
            )?;
        };

        if let Some(escrow_bump) = ctx.bumps.get("escrow_account"){
            orbit_transaction::close_escrow_sol_rate!(
                ctx.accounts.escrow_account.to_account_info(),
                ctx.accounts.seller_wallet.to_account_info(),
                &[&[b"orbit_escrow_account", ctx.accounts.digital_transaction.key().as_ref(), &[*escrow_bump]]],
                ctx.accounts.digital_transaction.metadata.rate
            ).expect("could not transfer tokens");
            orbit_transaction::close_escrow_sol_rate!(
                ctx.accounts.escrow_account.to_account_info(),
                ctx.accounts.buyer_wallet.to_account_info(),
                &[&[b"orbit_escrow_account", ctx.accounts.digital_transaction.key().as_ref(), &[*escrow_bump]]],
                100
            ).expect("could not transfer tokens");
        }else{
            return err!(DigitalMarketErrors::InvalidEscrowBump)
        };

        if let Some(escrow_seeds) = ctx.bumps.get("escrow_account"){
            orbit_transaction::close_escrow_sol_rate!(
                ctx.accounts.escrow_account.to_account_info(),
                ctx.accounts.buyer_wallet.to_account_info(),
                &[&[b"orbit_escrow_account", ctx.accounts.digital_transaction.key().as_ref(), &[*escrow_seeds]]],
                100
            )?;
        }else{
            return err!(DigitalMarketErrors::InvalidEscrowBump)
        };
        
        orbit_transaction::cpi::clear_from_seller_transactions_log(
            CpiContext::new(
                ctx.accounts.transaction_program.to_account_info(),
                orbit_transaction::cpi::accounts::ClearSellerTransactions{
                    transactions_log: ctx.accounts.seller_transactions_log.to_account_info(),
                    caller_auth: ctx.accounts.digital_auth.to_account_info(),
                    caller: ctx.accounts.digital_program.to_account_info()
                }
            ),
            ctx.accounts.digital_transaction.metadata.seller_tx_index
        )?;

        orbit_transaction::cpi::clear_from_buyer_transactions_log(
            CpiContext::new(
                ctx.accounts.transaction_program.to_account_info(),
                orbit_transaction::cpi::accounts::ClearBuyerTransactions{
                    transactions_log: ctx.accounts.buyer_transactions_log.to_account_info(),
                    caller_auth: ctx.accounts.digital_auth.to_account_info(),
                    caller: ctx.accounts.digital_program.to_account_info()
                }
            ),
            ctx.accounts.digital_transaction.metadata.seller_tx_index
        )?;
        Ok(())
    }

    fn seller_early_decline_spl(ctx: Context<SellerEarlyDeclineSpl>) -> Result<()>{
        ctx.accounts.digital_transaction.metadata.transaction_state = TransactionState::Closed;

        if ctx.accounts.digital_transaction.metadata.rate == 100{
            market_accounts::cpi::increment_dispute_discounts(
                CpiContext::new(
                    ctx.accounts.market_account_program.to_account_info(),
                    market_accounts::cpi::accounts::MarketAccountUpdateInternal{
                        market_account: ctx.accounts.buyer_account.to_account_info(),
                        caller_auth: ctx.accounts.digital_auth.to_account_info(),
                        caller: ctx.accounts.digital_program.to_account_info()
                    }
                )
            )?;
        }

        if let Some(auth_bump) = ctx.bumps.get("digital_auth"){
            orbit_transaction::close_escrow_spl_rate!(
                ctx.accounts.token_program.to_account_info(),
                ctx.accounts.escrow_account.to_account_info(),
                ctx.accounts.seller_token_account.to_account_info(),
                ctx.accounts.digital_auth.to_account_info(),
                &[&[b"market_authority", &[*auth_bump]]],
                ctx.accounts.digital_transaction.metadata.transaction_price,
                ctx.accounts.digital_transaction.metadata.rate
            ).expect("could not transfer tokens");
            orbit_transaction::close_escrow_spl_rate!(
                ctx.accounts.token_program.to_account_info(),
                ctx.accounts.escrow_account.to_account_info(),
                ctx.accounts.buyer_token_account.to_account_info(),
                ctx.accounts.digital_auth.to_account_info(),
                &[&[b"market_authority", &[*auth_bump]]],
                ctx.accounts.digital_transaction.metadata.transaction_price,
                100
            ).expect("could not transfer tokens");
        }else{
            return err!(DigitalMarketErrors::InvalidAuthBump)
        }; 
        
        orbit_transaction::cpi::clear_from_seller_transactions_log(
            CpiContext::new(
                ctx.accounts.transaction_program.to_account_info(),
                orbit_transaction::cpi::accounts::ClearSellerTransactions{
                    transactions_log: ctx.accounts.seller_transactions_log.to_account_info(),
                    caller_auth: ctx.accounts.digital_auth.to_account_info(),
                    caller: ctx.accounts.digital_program.to_account_info()
                }
            ),
            ctx.accounts.digital_transaction.metadata.seller_tx_index
        )?;

        orbit_transaction::cpi::clear_from_buyer_transactions_log(
            CpiContext::new(
                ctx.accounts.transaction_program.to_account_info(),
                orbit_transaction::cpi::accounts::ClearBuyerTransactions{
                    transactions_log: ctx.accounts.buyer_transactions_log.to_account_info(),
                    caller_auth: ctx.accounts.digital_auth.to_account_info(),
                    caller: ctx.accounts.digital_program.to_account_info()
                }
            ),
            ctx.accounts.digital_transaction.metadata.seller_tx_index
        )?;

        Ok(())

    }

}

//////////////////////////////////////////////////////////////////////////
/// BUYER CONFIRMATIONS

#[derive(Accounts)]
pub struct BuyerConfirmation<'info>{
    #[account(
        mut,
        constraint = digital_transaction.final_decision == BuyerDecisionState::Null,
    )]
    pub digital_transaction: Box<Account<'info, DigitalTransaction>>,

    #[account(
        address = digital_transaction.metadata.buyer,
        has_one = buyer_wallet
    )]
    pub buyer_transactions: Box<Account<'info, BuyerOpenTransactions>>,

    pub buyer_wallet: Signer<'info>,
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
    ctx.accounts.digital_transaction.final_decision = BuyerDecisionState::Accept;
    // we dont set state here because we need to wait for the seller to release the final keys
    Ok(())
}

#[derive(Accounts)]
pub struct BuyerDeny<'info>{
    #[account(
        mut,
        constraint = digital_transaction.final_decision == BuyerDecisionState::Null,
    )]
    pub digital_transaction: Box<Account<'info, DigitalTransaction>>,

    #[account(
        mut
    )]
    pub buyer_account: Account<'info, OrbitMarketAccount>,

    #[account(
        address = digital_transaction.metadata.buyer,
        has_one = buyer_wallet
    )]
    pub buyer_transactions: Box<Account<'info, BuyerOpenTransactions>>,

    #[account(
        address = buyer_account.wallet
    )]
    pub buyer_wallet: Signer<'info>,
    
    #[account(
        seeds = [b"market_authority"],
        bump
    )]
    pub digital_auth: SystemAccount<'info>,

    pub digital_program: Program<'info, OrbitDigitalMarket>,

    pub market_accounts_program: Program<'info, OrbitMarketAccounts>
}

pub fn deny_accept_handler(ctx: Context<BuyerDeny>) -> Result<()>{
    if ctx.accounts.digital_transaction.metadata.transaction_state != TransactionState::BuyerConfirmedDelivery{
        return err!(DigitalMarketErrors::DidNotConfirmDelivery);
    }
    if ctx.accounts.digital_transaction.metadata.rate == 100{
        market_accounts::cpi::increment_dispute_discounts(
            CpiContext::new(
                ctx.accounts.market_accounts_program.to_account_info(),
                market_accounts::cpi::accounts::MarketAccountUpdateInternal{
                    market_account: ctx.accounts.buyer_account.to_account_info(),
                    caller_auth: ctx.accounts.digital_auth.to_account_info(),
                    caller: ctx.accounts.digital_program.to_account_info()
                }
            )
        )?;
    }
    ctx.accounts.digital_transaction.metadata.rate = 0;
    ctx.accounts.digital_transaction.final_decision = BuyerDecisionState::Declined;
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
    )]
    pub seller_transactions: Box<Account<'info, SellerOpenTransactions>>,

    #[account(
        address = seller_transactions.seller_wallet
    )]
    pub wallet: Signer<'info>
}

pub fn seller_accept_transaction_handler(ctx: Context<SellerAcceptTransaction>) -> Result<()>{
    ctx.accounts.digital_transaction.metadata.transaction_state = TransactionState::SellerConfirmed;
    Ok(())
}

#[derive(Accounts)]
pub struct CommitInitData<'info>{
    #[account(
        mut,
        constraint = digital_transaction.metadata.transaction_state == TransactionState::BuyerFunded
    )]
    pub digital_transaction: Box<Account<'info, DigitalTransaction>>,

    #[account(
        address = digital_transaction.metadata.seller,
        has_one = seller_wallet
    )]
    pub seller_transactions: Box<Account<'info, SellerOpenTransactions>>,

    pub seller_wallet: Signer<'info>,
}

pub fn commit_init_keys_handler(ctx: Context<CommitInitData>, submission_keys: Vec<Pubkey>) -> Result<()>{   
    if (submission_keys.len() > 64) || (submission_keys.len() <= 0){
        return err!(DigitalMarketErrors::IndexOutOfRange)
    }

    let mut total_keys: u64 = 0;
    for ind in 0..submission_keys.len(){
        total_keys |= 1<<ind;
    }
    ctx.accounts.digital_transaction.num_keys = total_keys;
    ctx.accounts.digital_transaction.key_arr = submission_keys;
    Ok(())
}

pub fn commit_link_handler(ctx: Context<CommitInitData>, link: String) -> Result<()>{
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
        constraint = digital_transaction.metadata.transaction_state == TransactionState::BuyerConfirmedDelivery
    )]
    pub digital_transaction: Box<Account<'info, DigitalTransaction>>,

    #[account(
        address = digital_transaction.metadata.seller,
        has_one = seller_wallet
    )]
    pub seller_transactions: Box<Account<'info, SellerOpenTransactions>>,

    pub seller_wallet: Signer<'info>,
}

pub fn commit_subkeys_handler(ctx: Context<CommitSubKeys>, indexes: Vec<u8>) -> Result<()>{
    for index in indexes{
        if index > ctx.accounts.digital_transaction.key_arr.len() as u8{
            return err!(DigitalMarketErrors::IndexOutOfRange)
        }

        let acc = &ctx.remaining_accounts[index as usize];
        if ! acc.is_signer{
            return err!(DigitalMarketErrors::CorruptPrivateKeyFormat);
        }

        if Pubkey::find_program_address(&[acc.key().as_ref()], &id()).0 != ctx.accounts.digital_transaction.key_arr[index as usize]{
            return err!(DigitalMarketErrors::IncorrectPrivateKey);
        }

        ctx.accounts.digital_transaction.num_keys &= (!(1 << index)) as u64;
        ctx.accounts.digital_transaction.key_arr[index as usize] = acc.key();
    }

    if ctx.accounts.digital_transaction.num_keys == 0{
        ctx.accounts.digital_transaction.metadata.transaction_state = TransactionState::BuyerConfirmedProduct;
    }

    Ok(())
}

///////////////////////////////////////////////////////////////////////
/// ACCOUNT HELPERS (leave a review!)

#[derive(Accounts)]
pub struct LeaveReview<'info>{
    /////////////////////////////////////////////////
    /// TX
    #[account(
        mut,
        constraint = digital_transaction.metadata.transaction_state == TransactionState::Closed
    )]
    pub digital_transaction: Box<Account<'info, DigitalTransaction>>,

    /////////////////////////////////////////////////
    /// REVIEW RELATED
    #[account(
        mut,
        constraint = 
        (reviewer.seller_physical_transactions == digital_transaction.metadata.seller) ||
        (reviewer.buyer_physical_transactions == digital_transaction.metadata.buyer)
    )]
    pub reviewed_account: Box<Account<'info, OrbitMarketAccount>>,

    #[account(
        constraint = 
        (reviewer.seller_physical_transactions == digital_transaction.metadata.seller) ||
        (reviewer.buyer_physical_transactions == digital_transaction.metadata.buyer),
        seeds = [
            b"orbit_account",
            wallet.key().as_ref()
        ],
        bump,
        seeds::program = market_accounts::ID
    )]
    pub reviewer: Box<Account<'info, OrbitMarketAccount>>,

    #[account(
        address = reviewer.wallet
    )]
    pub wallet: Signer<'info>,

    /////////////////////////////////////////////////
    /// EXTRANEOUS CPI
    #[account(
        seeds = [b"market_authority"],
        bump
    )]
    pub digital_auth: SystemAccount<'info>,
    
    pub digital_program: Program<'info, OrbitDigitalMarket>,

    pub accounts_program: Program<'info, OrbitMarketAccounts>
}

impl <'a> OrbitMarketAccountTrait<'a, LeaveReview<'a>> for DigitalTransaction{
    fn leave_review(ctx: Context<LeaveReview>, rating: u8) -> Result<()>{
        if ctx.accounts.reviewer.key() == ctx.accounts.reviewed_account.key(){
            return err!(ReviewErrors::InvalidReviewAuthority)
        };
        if rating == 0 || rating > 5{
            return err!(ReviewErrors::RatingOutsideRange)
        };

        if ctx.accounts.digital_transaction.metadata.seller == ctx.accounts.reviewer.key() && !ctx.accounts.digital_transaction.metadata.reviews.seller{
            if let Some(auth_bump) = ctx.bumps.get("digital_auth"){
                orbit_transaction::submit_rating_with_signer!(
                    ctx.accounts.accounts_program.to_account_info(),
                    ctx.accounts.reviewed_account.to_account_info(),
                    ctx.accounts.digital_auth.to_account_info(),
                    ctx.accounts.digital_program.to_account_info(),
                    &[&[b"market_authority", &[*auth_bump]]],
                    rating
                )?;
                ctx.accounts.digital_transaction.metadata.reviews.seller = true;
            }else{
                return err!(MarketAccountErrors::CannotCallOrbitAccountsProgram)
            };
        }else
        if ctx.accounts.digital_transaction.metadata.buyer == ctx.accounts.reviewer.key() && !ctx.accounts.digital_transaction.metadata.reviews.buyer{
            if let Some(auth_bump) = ctx.bumps.get("digital_auth"){
                orbit_transaction::submit_rating_with_signer!(
                    ctx.accounts.accounts_program.to_account_info(),
                    ctx.accounts.reviewed_account.to_account_info(),
                    ctx.accounts.digital_auth.to_account_info(),
                    ctx.accounts.digital_program.to_account_info(),
                    &[&[b"market_authority", &[*auth_bump]]],
                    rating
                )?;
                ctx.accounts.digital_transaction.metadata.reviews.buyer = true;
                 
            }else{
                return err!(MarketAccountErrors::CannotCallOrbitAccountsProgram)
            }
        }else
        {
            return err!(ReviewErrors::InvalidReviewAuthority)
        };

        Ok(())
    }

}