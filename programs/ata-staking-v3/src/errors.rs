use anchor_lang::prelude::*;

#[error_code]
pub enum AtaSkakingError {
  #[msg("unknown error")]
  UnknownError,
}