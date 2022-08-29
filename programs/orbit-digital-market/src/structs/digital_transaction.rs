use anchor_lang::prelude::*;
use market_accounts::structs::TransactionReviews;
use transaction::transaction_struct::OrbitTransaction;

#[account]
pub struct DigitalTransaction{
    pub metadata: OrbitTransaction, // 170
    pub product: Pubkey, // 32

    pub close_rate: u8, // 1

    pub has_comish: bool, // 1
    pub comish_account: Pubkey, // 32

    pub data_address: [u8; 64], // 64
    pub num_keys: u64, // 8
    pub key_arr: Vec<Pubkey>, // up to 2048
    pub final_decision: BuyerDecisionState, // 1

    pub reviews: TransactionReviews // 1
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
    Declined,
    Accept
}