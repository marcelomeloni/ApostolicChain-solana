use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("O parent_hash não é 00x00x00, então você DEVE fornecer a conta do pai para validação.")]
    ParentAccountMissing,
    #[msg("A conta do pai fornecida não corresponde ao parent_hash informado.")]
    InvalidParentHash,
}