use anchor_lang::error_code;

#[error_code]
pub enum StakeError {
    #[msg("Freeze period not passed!")]
    FreezePeriodNotPassed,
    #[msg("Maximum stake reached!")]
    MaxStakeReached,
}