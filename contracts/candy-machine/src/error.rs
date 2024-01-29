use base64::DecodeError;
use cosmwasm_std::{OverflowError, StdError, Uint128, VerificationError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Overflow(#[from] OverflowError),

    #[error(transparent)]
    Decode(#[from] DecodeError),

    #[error(transparent)]
    CryptoVerify(#[from] VerificationError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Nft address is not defined")]
    NftAddressNotDefined {},

    #[error("Unknown mint stage")]
    UnknownMintStage {},

    #[error("No more nfts to mint")]
    NoMoreNftsToMint {},

    #[error("Mint amount per user exceeded")]
    MaximumMintAmountPerUserExceeded {},

    #[error("Mint not started. (time: {start:?})")]
    MintNotStarted { start: u64 },

    #[error("Mint finished. (time: {finish:?})")]
    MintFinished { finish: u64 },

    #[error("{address:?} is not whitelisted for mint")]
    NotAllowNonWhitelisted { address: String },

    #[error("Zero amount not allowed")]
    NotAllowZeroAmount {},

    #[error("Other denom except {denom:?} is not allowed")]
    NotAllowOtherDenoms { denom: String },

    #[error("Invalid amount, expected amount: {amount:?}")]
    InvalidAmount { amount: Uint128 },

    #[error("Invalid collection kind")]
    InvalidCollectionKind {},

    #[error("Unknown reservation")]
    UnknownReservation {},

    #[error("Invalid signature")]
    InvalidSignature {},
}
