use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    system_instruction::transfer,
    program::invoke_signed
};
use market_accounts::cpi::accounts::PostTxContext;


pub fn close_escrow_sol<'a>(escrow_account: AccountInfo<'a>, destination: AccountInfo<'a>, seeds: &[&[&[u8]]], rate: u8) -> Result<()>{
    invoke_signed(
        &transfer(
            &escrow_account.key(),
            &destination.key(),
            (escrow_account.lamports() * rate as u64) / 100
        ),
        &[
            escrow_account,
            destination
        ],
        seeds
    ).map_err(|e| anchor_lang::error::Error::from(e))
}


pub fn close_escrow_spl<'a>(token_program: AccountInfo<'a>, escrow_account: AccountInfo<'a>, destination: AccountInfo<'a>, digital_auth: AccountInfo<'a>, seeds: &[&[&[u8]]], price: u64, rate: u8) -> Result<()>{
    anchor_spl::token::transfer(
        CpiContext::new_with_signer(
            token_program, 
            anchor_spl::token::Transfer{
                from: escrow_account,
                to: destination,
                authority: digital_auth
            },
            seeds
        ),
        (price * rate as u64)/100
    ).expect("could not close transaction");
    Ok(())
}

pub fn post_tx_incrementing<'a>(account_program: AccountInfo<'a>, buyer_acc: AccountInfo<'a>, seller_acc: AccountInfo<'a>, digital_auth: AccountInfo<'a>, digital_program: AccountInfo<'a>, seeds: &[&[&[u8]]]) -> Result<()>{
    // we gotta clone :/
    market_accounts::cpi::post_tx(
        CpiContext::new_with_signer(
            account_program.to_account_info(),
            PostTxContext{
                market_account: buyer_acc.to_account_info(),
                caller_auth: digital_auth.to_account_info(),
                caller: digital_program.to_account_info()
            },
            seeds
        )
    ).expect("could not properly invoke market-accounts program");
    market_accounts::cpi::post_tx(
        CpiContext::new_with_signer(
            account_program,
            PostTxContext{
                market_account: seller_acc,
                caller_auth: digital_auth,
                caller: digital_program
            },
            seeds
        )
    )
}

/// CHECK: has to be cpi because we can't write to a program we dont own (physical writing to market account directly)
pub fn submit_rating_with_signer<'a>(market_program: AccountInfo<'a>, reviewed_account: AccountInfo<'a>, digital_auth: AccountInfo<'a>, digital_program: AccountInfo<'a>, seeds: &[&[&[u8]]], rating: u8){
    market_accounts::cpi::submit_rating(
        CpiContext::new_with_signer(
            market_program,
            PostTxContext{
                market_account: reviewed_account,
                caller_auth: digital_auth,
                caller: digital_program
            },
            seeds
        ),
        (rating-1) as usize
    ).expect("could not call orbit accounts program");
}