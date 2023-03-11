use anchor_lang::prelude::*;

#[error_code]
pub enum AtaSkakingError {
  #[msg("unknown error")]
  UnknownError,
}

#[error_code]
pub enum TimeError {
    #[msg("InvalidTime")]
    InvalidTime,
    #[msg("InvalidEpoch")]
    InvalidEpoch
}

#[error_code]
pub enum ConditionError {
  #[msg("InvalidCondition")]
  InvalidCondition
}