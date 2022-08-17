use anchor_lang::prelude::*;

#[error_code]
pub enum DigitalMarketErrors{
    #[msg("Wrong account to pay")]
    WrongDecisionAccount,
    #[msg("Decision has not been made yet, can not close")]
    UndecidedTransaction,
    #[msg("Could not compute escrow bump")]
    InvalidEscrowBump,
    #[msg("Can not accept your own rate")]
    InvalidRateAcceptor,
    #[msg("Please confirm delivery first")]
    DidNotConfirmDelivery,
    #[msg("The seller did not commit keys yet")]
    WaitingForSellerData,
    #[msg("The seller for the product does not match the seller given")]
    InvalidSellerForListing,
}