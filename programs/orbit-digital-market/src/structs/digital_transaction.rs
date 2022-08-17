use anchor_lang::prelude::*;
use transaction::transaction_struct::OrbitTransaction;

#[account]
pub struct DigitalTransaction{
    pub metadata: OrbitTransaction,
    pub product: Pubkey,

    pub close_rate: u8,

    // free the comish
    pub has_comish: bool,
    pub comish_account: Pubkey,

    // no matter template or commission, this is always the final step
    pub data_address: [u8; 64],
    pub keys_array: [[u8; 16]; 64], // aes 128 bit keys, up to 64 of them
    pub final_decision: BuyerDecisionState,
}

#[account]
pub struct ComishAccount{
    pub preview_address: [u8; 64],
    pub preview_rate: u8,
    pub last_rate_offerer: Pubkey,

    pub funder: Pubkey,
}

// goes buyer conf. delivery -> buyer accepts/denies
// if accepts -> seller releases key and closes. escrow to seller. 
// keys are precommited so they cant scam
// if denied -> escrow to buyer (10% still goes to seller maybe)
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum BuyerDecisionState{
    Null,
    Decided
}