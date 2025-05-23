use anchor_lang::prelude::*;
use orbit_transaction::transaction_struct::OrbitTransactionStruct;

#[account]
pub struct DigitalTransaction{
    pub metadata: OrbitTransactionStruct, // 172

    pub data_address: String, // 64
    pub num_keys: u64, // 8
    pub key_arr: Vec<Pubkey>, // up to 2048
    pub final_decision: BuyerDecisionState, // 1
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